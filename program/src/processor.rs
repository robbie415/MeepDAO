use self::{
    initialize_meep::process_initialize, mint_nft::process_mint,
    update_settings::process_update_settings,
};
use crate::instruction::MeepInstructions;
use borsh::BorshDeserialize;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey};

mod initialize_meep;
mod mint_nft;
mod update_settings;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = MeepInstructions::try_from_slice(instruction_data)?;
    match instruction {
        MeepInstructions::InitializeMeep(settings) => {
            process_initialize(program_id, accounts, settings)
        }
        MeepInstructions::UpdateSettings(settings) => {
            process_update_settings(program_id, accounts, settings)
        }
        MeepInstructions::MintNft(mint_args) => process_mint(program_id, accounts, mint_args),
    }
}
