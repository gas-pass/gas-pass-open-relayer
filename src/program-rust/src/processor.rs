use crate::{error::GasPassError, instruction::GasPassInstruction, state::GasPassInfo};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack},
    pubkey::Pubkey,
    rent::Rent,
    sysvar::Sysvar,
};
use std::collections::HashMap;

/// Program state processor.
pub struct Processor;

impl Processor {
    /// Processes an [Instruction](enum.Instruction.html).
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = GasPassInstruction::try_from_slice(instruction_data)?;

        match instruction {
            GasPassInstruction::InitializeAccount { nonce } => {
                msg!("Instruction: InitializeAccount");
                Self::process_initialize_account(accounts, nonce, program_id)
            }
            GasPassInstruction::Topup { amount } => {
                msg!("Instruction: Topup");
                Self::process_topup(accounts, amount, program_id)
            }
            GasPassInstruction::SubmitTransaction { transaction } => {
                msg!("Instruction: SubmitTransaction");
                Self::process_submit_transaction(accounts, transaction, program_id)
            }
            GasPassInstruction::Withdraw { amount } => {
                msg!("Instruction: Withdraw");
                Self::process_withdraw(accounts, amount, program_id)
            }
        }
    }

    pub fn process_initialize_account(
        accounts: &[AccountInfo],
        nonce: u8,
        program_id: &Pubkey,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let gas_pass_account = next_account_info(account_info_iter)?;
        let authority = next_account_info(account_info_iter)?;
        let rent = &Rent::from_account_info(next_account_info(account_info_iter)?)?;

        if !authority.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        if gas_pass_account.owner != program_id {
            return Err(ProgramError::IncorrectProgramId);
        }

        if !rent.is_exempt(gas_pass_account.lamports(), GasPassInfo::get_account_len()) {
            return Err(GasPassError::NotRentExempt.into());
        }

        let mut gas_pass_info = GasPassInfo::try_from_slice(&gas_pass_account.data.borrow())?;
        if gas_pass_info.is_initialized() {
            return Err(GasPassError::AccountAlreadyInUse.into());
        }

        gas_pass_info.is_initialized = true;
        gas_pass_info.authority = *authority.key;
        gas_pass_info.nonce = nonce;
        gas_pass_info.balance = 0;

        gas_pass_info.serialize(&mut &mut gas_pass_account.data.borrow_mut()[..])?;
        msg!("GAS PASS account initialized");
        Ok(())
    }

    pub fn process_topup(
        accounts: &[AccountInfo],
        amount: u64,
        program_id: &Pubkey,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let gas_pass_account = next_account_info(account_info_iter)?;
        let authority = next_account_info(account_info_iter)?;
        let payer = next_account_info(account_info_iter)?;

        if !authority.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        if gas_pass_account.owner != program_id {
            return Err(ProgramError::IncorrectProgramId);
        }

        let mut gas_pass_info = GasPassInfo::try_from_slice(&gas_pass_account.data.borrow())?;
        if !gas_pass_info.is_initialized() {
            return Err(GasPassError::AccountNotInitialized.into());
        }

        if gas_pass_info.authority != *authority.key {
            return Err(GasPassError::OwnerMismatch.into());
        }

        // Transfer lamports from payer to gas pass account
        **gas_pass_account.try_borrow_mut_lamports()? += amount;
        **payer.try_borrow_mut_lamports()? -= amount;

        gas_pass_info.balance += amount;
        gas_pass_info.serialize(&mut &mut gas_pass_account.data.borrow_mut()[..])?;

        msg!("Topup successful: {} lamports added", amount);
        Ok(())
    }

    pub fn process_submit_transaction(
        accounts: &[AccountInfo],
        transaction: Vec<u8>,
        program_id: &Pubkey,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let gas_pass_account = next_account_info(account_info_iter)?;
        let executor = next_account_info(account_info_iter)?;

        if gas_pass_account.owner != program_id {
            return Err(ProgramError::IncorrectProgramId);
        }

        let mut gas_pass_info = GasPassInfo::try_from_slice(&gas_pass_account.data.borrow())?;
        if !gas_pass_info.is_initialized() {
            return Err(GasPassError::AccountNotInitialized.into());
        }

        // Calculate transaction fee
        let fee = 50000; // Base fee for transaction execution

        if gas_pass_info.balance < fee {
            return Err(GasPassError::AmountOverflow.into());
        }

        // Deduct fee from gas pass account
        gas_pass_info.balance -= fee;
        gas_pass_info.serialize(&mut &mut gas_pass_account.data.borrow_mut()[..])?;

        // Transfer fee to executor
        **gas_pass_account.try_borrow_mut_lamports()? -= fee;
        **executor.try_borrow_mut_lamports()? += fee;

        msg!("Transaction submitted successfully. Fee: {} lamports", fee);
        Ok(())
    }

    pub fn process_withdraw(
        accounts: &[AccountInfo],
        amount: u64,
        program_id: &Pubkey,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let gas_pass_account = next_account_info(account_info_iter)?;
        let authority = next_account_info(account_info_iter)?;

        if !authority.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        if gas_pass_account.owner != program_id {
            return Err(ProgramError::IncorrectProgramId);
        }

        let mut gas_pass_info = GasPassInfo::try_from_slice(&gas_pass_account.data.borrow())?;
        if !gas_pass_info.is_initialized() {
            return Err(GasPassError::AccountNotInitialized.into());
        }

        if gas_pass_info.authority != *authority.key {
            return Err(GasPassError::OwnerMismatch.into());
        }

        if gas_pass_info.balance < amount {
            return Err(GasPassError::AmountOverflow.into());
        }

        // Transfer lamports from gas pass account to authority
        **gas_pass_account.try_borrow_mut_lamports()? -= amount;
        **authority.try_borrow_mut_lamports()? += amount;

        gas_pass_info.balance -= amount;
        gas_pass_info.serialize(&mut &mut gas_pass_account.data.borrow_mut()[..])?;

        msg!("Withdrawal successful: {} lamports withdrawn", amount);
        Ok(())
    }
}

impl PrintProgramError for GasPassError {
    fn print<E>(&self)
    where
        E: 'static + std::error::Error + DecodeError<E> + PrintProgramError + FromPrimitive,
    {
        match self {
            GasPassError::AccountAlreadyInUse => info!("Error: GAS PASS account already in use"),
            GasPassError::InvalidInstruction => info!("Error: Invalid instruction"),
            GasPassError::NotRentExempt => info!("Error: Account not rent exempt"),
            GasPassError::ExpectedAmountMismatch => info!("Error: Expected amount mismatch"),
            GasPassError::AmountOverflow => info!("Error: Amount overflow"),
            GasPassError::AccountNotInitialized => info!("Error: Account not initialized"),
            GasPassError::OwnerMismatch => info!("Error: Owner mismatch"),
        }
    }
}
