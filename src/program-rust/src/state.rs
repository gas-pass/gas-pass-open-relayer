use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    program_pack::{IsInitialized, Pack},
    pubkey::Pubkey,
};
use std::collections::HashMap;

/// GAS PASS account information
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct GasPassInfo {
    /// Whether this account has been initialized
    pub is_initialized: bool,
    /// The authority of this account
    pub authority: Pubkey,
    /// Nonce for the account
    pub nonce: u8,
    /// Current balance in lamports
    pub balance: u64,
}

impl GasPassInfo {
    /// Get the length of the account data
    pub fn get_account_len() -> usize {
        std::mem::size_of::<Self>()
    }
}

impl IsInitialized for GasPassInfo {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for GasPassInfo {
    const LEN: usize = std::mem::size_of::<Self>();

    fn unpack_from_slice(src: &[u8]) -> Result<Self, solana_program::program_error::ProgramError> {
        Self::try_from_slice(src).map_err(|_| solana_program::program_error::ProgramError::InvalidAccountData)
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        self.serialize(&mut dst).unwrap();
    }
}
