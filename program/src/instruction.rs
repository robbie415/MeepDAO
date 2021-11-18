use crate::utils::Pda;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_program,
    sysvar::{self, rent},
};

#[derive(BorshSerialize, BorshDeserialize, Clone)]
pub struct SettingsArgs {
    pub primary_wallet_percentage: u8, // [0-100]
    pub minimum_price: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Clone)]
pub struct MintNftArgs {
    pub seller_fee_basis_points: u16,
    pub token_name: String,
    pub token_symbol: String,
    pub uri: String,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub enum NounsInstructions {
    /// 0. `[signer, writable]` Authority (Primary creator, Payer)
    /// 1. `[signer]` Secondary creator
    /// 2. `[writable]` Settings account, PDA("settings_nouns", authority, program_id)
    /// 3. `[]` System program
    /// 4. `[]` Rent program
    InitializeNouns(SettingsArgs),

    /// 0. `[signer]` Authority (Primary creator, Payer)
    /// 1. `[writable]` Settings account, PDA("settings_nouns", authority, program_id)
    UpdateSettings(SettingsArgs),

    /// 0. `[signer, writable]` Authority (Primary creator, Payer)
    /// 1. `[signer]` Secondary creator
    /// 2. `[]` Settings account, PDA("settings_nouns", authority, program_id)
    /// 3. `[signer, writable]` Mint account  (Uninitialized)
    /// 4. `[signer, writable]` Token account (Uninitialized)
    /// 5. `[writable]` TokenMetadata account (Uninitialized)
    /// 6. `[writable]` MasterEdition account (Uninitialized)
    /// 7. `[]` System program
    /// 8. `[]` Token program
    /// 9. `[]` Rent program
    /// 10. `[]` Metaplex program
    MintNft(MintNftArgs),
}

impl NounsInstructions {
    pub fn initialize_nouns(
        program_id: &Pubkey,
        authority: &Pubkey,
        secondary_creator: &Pubkey,
        args: &SettingsArgs,
    ) -> Instruction {
        let settings_pubkey = Pda::settings_pubkey_with_bump(program_id, authority).0;

        Instruction::new_with_borsh(
            *program_id,
            &NounsInstructions::InitializeNouns(args.clone()),
            vec![
                AccountMeta::new(*authority, true),
                AccountMeta::new_readonly(*secondary_creator, true),
                AccountMeta::new(settings_pubkey, false),
                AccountMeta::new_readonly(system_program::ID, false),
                AccountMeta::new_readonly(rent::ID, false),
            ],
        )
    }

    pub fn update_settings(
        program_id: &Pubkey,
        authority: &Pubkey,
        args: &SettingsArgs,
    ) -> Instruction {
        let settings_pubkey = Pda::settings_pubkey_with_bump(program_id, authority).0;

        Instruction::new_with_borsh(
            *program_id,
            &NounsInstructions::UpdateSettings(args.clone()),
            vec![
                AccountMeta::new_readonly(*authority, true),
                AccountMeta::new(settings_pubkey, false),
            ],
        )
    }

    pub fn mint_nft(
        program_id: &Pubkey,
        authority: &Pubkey,
        secondary_creator: &Pubkey,
        mint: &Pubkey,
        token_account: &Pubkey,
        mint_args: &MintNftArgs,
    ) -> Instruction {
        let metadata = Pda::metadata_pubkey(mint);
        let edition = Pda::master_edition_pubkey(mint);
        let settings = Pda::settings_pubkey_with_bump(program_id, authority).0;

        Instruction::new_with_borsh(
            *program_id,
            &NounsInstructions::MintNft(mint_args.clone()),
            vec![
                AccountMeta::new(*authority, true),
                AccountMeta::new_readonly(*secondary_creator, true),
                AccountMeta::new_readonly(settings, false),
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
