/// Instructions supported by the GAS PASS.
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::program_error::ProgramError;
use std::mem::size_of;

/// Topup argument structure
#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub struct TopupAgrs {
    pub amount: u64,
}

/// Submit argument structure
#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub struct SubmitArgs {
    pub amount: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub enum GasPassInstruction {
    /// Initialize a new GAS PASS account
    /// 
    /// Accounts expected:
    /// 0. `[writable]` The account to initialize
    /// 1. `[signer]` The authority of the account
    /// 2. `[]` Rent sysvar
    InitializeAccount {
        /// Nonce for the account
        nonce: u8,
    },

    /// Top up the GAS PASS account
    /// 
    /// Accounts expected:
    /// 0. `[writable]` The GAS PASS account
    /// 1. `[signer]` The authority of the account
    /// 2. `[writable]` The payer account
    Topup {
        /// Amount to top up
        amount: u64,
    },

    /// Submit a transaction for execution
    /// 
    /// Accounts expected:
    /// 0. `[writable]` The GAS PASS account
    /// 1. `[writable]` The executor account
    SubmitTransaction {
        /// Transaction data
        transaction: Vec<u8>,
    },

    /// Withdraw funds from the GAS PASS account
    /// 
    /// Accounts expected:
    /// 0. `[writable]` The GAS PASS account
    /// 1. `[signer]` The authority of the account
    Withdraw {
        /// Amount to withdraw
        amount: u64,
    },
}

impl GasPassInstruction {
    pub fn deserialize(input: &[u8]) -> Result<Self, ProgramError> {
        if input.len() < size_of::<u8>() {
            return Err(ProgramError::InvalidAccountData);
        }
        Ok(match input[0] {
            0 => Self::InitializeAccount { nonce: 0 },
            1 => {
                let val: &TopupAgrs = unpack(input)?;
                Self::Topup { amount: val.amount }
            }
            2 => {
                let val: &SubmitArgs = unpack(input)?;
                Self::SubmitTransaction { transaction: Vec::new() }
            }
            3 => Self::Withdraw { amount: 0 },
            _ => return Err(ProgramError::InvalidAccountData),
        })
    }
}

/// Unpacks a reference from a bytes buffer.
pub fn unpack<T>(input: &[u8]) -> Result<&T, ProgramError> {
    if input.len() < size_of::<u8>() + size_of::<T>() {
        return Err(ProgramError::InvalidAccountData);
    }
    #[allow(clippy::cast_ptr_alignment)]
    let val: &T = unsafe { &*(&input[1] as *const u8 as *const T) };
    Ok(val)
}
