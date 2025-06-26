use crate::{error::GasPassError, processor::Processor};
use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, msg, pubkey::Pubkey,
};

// Declare and export the program's entrypoint
entrypoint!(process_instruction);

// Program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey, // Public key of the account the hello world program was loaded into
    accounts: &[AccountInfo], // The account to say hello to
    instruction_data: &[u8], // Ignored, all helloworld instructions are hellos
) -> ProgramResult {
    msg!("GAS PASS program entrypoint");
    let processor = Processor {};
    processor.process(program_id, accounts, instruction_data)?;
    Ok(())
}

// Sanity check
#[cfg(test)]
mod test {
    use super::*;
    use solana_program::clock::Epoch;
    use solana_program::pubkey::Pubkey;
    use solana_program::sysvar::{rent::Rent, Sysvar};
    use solana_program::transaction::{AccountMeta, Transaction};
    use solana_program::{account_info::AccountInfo, instruction::Instruction};
    use solana_program::{bpf_loader, system_program};
    use solana_program::{program_error::ProgramError, program_pack::Pack};
    use solana_program::{program_stubs, rent::Rent};
    use solana_program::{sysvar::rent::Rent, Sysvar};
    use std::mem;

    program_stubs!();

    #[test]
    fn test_helloworld() {
        let program_id = Pubkey::new_unique();
        let key = Pubkey::new_unique();
        let mut lamports = 0;
        let mut data = vec![0; mem::size_of::<u64>()];
        let owner = Pubkey::new_unique();
        let account = AccountInfo::new(
            &key,
            false,
            true,
            &mut lamports,
            &mut data,
            &owner,
            false,
            Epoch::default(),
        );
        let instruction_data: Vec<u8> = Vec::new();

        let accounts = vec![account];

        assert_eq!(
            GasPassError::AccountAlreadyInUse,
            process_instruction(&program_id, &accounts, &instruction_data).unwrap_err()
        );
    }
}
