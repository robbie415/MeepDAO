use self::{
    initialize_nouns::process_initialize, mint_nft::process_mint,
    update_settings::process_update_settings,
};
use crate::instruction::NounsInstructions;
use borsh::BorshDeserialize;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey};

mod initialize_nouns;
mod mint_nft;
mod update_settings;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = NounsInstructions::try_from_slice(instruction_data)?;
    match instruction {
        NounsInstructions::InitializeNouns(settings) => {
            process_initialize(program_id, accounts, settings)
        }
        NounsInstructions::UpdateSettings(settings) => {
            process_update_settings(program_id, accounts, settings)
        }
        NounsInstructions::MintNft(mint_args) => process_mint(program_id, accounts, mint_args),
    }
}
