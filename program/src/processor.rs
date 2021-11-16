use self::mint_nft::process_mint;
use crate::instruction::NounsInstructions;
use borsh::BorshDeserialize;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey};

mod mint_nft;

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = NounsInstructions::try_from_slice(instruction_data)?;
    match instruction {
        NounsInstructions::MintNft(mint_args) => process_mint(accounts, mint_args),
    }
}
