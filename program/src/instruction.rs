use crate::utils::Pda;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_program, sysvar,
};

#[derive(BorshSerialize, BorshDeserialize, Clone)]
pub struct MintNftArgs {
    pub seller_fee_basis_points: u16,
    pub token_name: String,
    pub token_symbol: String,
    pub uri: String,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub enum NounsInstructions {
    /// 0. `[signer, writable]` Creator account (payer)
    /// 1. `[signer, writable]` Mint account
    /// 2. `[signer, writable]` Token account
    /// 3. `[writable]` TokenMetadata account
    /// 4. `[writable]` MasterEdition account
    /// 5. `[]` System program
    /// 6. `[]` Token program
    /// 7. `[]` Rent program
    /// 8. `[]` Metaplex program
    MintNft(MintNftArgs),
}

impl NounsInstructions {
    pub fn mint_nft(
        program_id: &Pubkey,
        creator: &Pubkey,
        mint: &Pubkey,
        token_account: &Pubkey,
        mint_args: &MintNftArgs,
    ) -> Instruction {
        let metadata = Pda::metadata_pubkey(mint);
        let edition = Pda::master_edition_pubkey(mint);

        Instruction::new_with_borsh(
            *program_id,
            &NounsInstructions::MintNft(mint_args.clone()),
            vec![
                AccountMeta::new(*creator, true),
                AccountMeta::new(*mint, true),
                AccountMeta::new(*token_account, true),
                AccountMeta::new(metadata, false),
                AccountMeta::new(edition, false),
                AccountMeta::new_readonly(system_program::ID, false),
                AccountMeta::new_readonly(spl_token::ID, false),
                AccountMeta::new_readonly(sysvar::rent::ID, false),
                AccountMeta::new_readonly(metaplex_token_metadata::ID, false),
            ],
        )
    }
}
