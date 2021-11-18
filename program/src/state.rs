use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(BorshSerialize, BorshDeserialize)]
pub struct NounsSettings {
    pub authority: Pubkey,
    pub secondary_creator: Pubkey,

    pub primary_wallet_percentage: u8,
    pub minimum_price: u64,
}
