use crate::{
    instruction::SettingsArgs,
    state::MeepSettings,
    utils::{assert_authority, get_settings_checked},
};
use borsh::BorshSerialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    pubkey::Pubkey,
};

pub fn process_update_settings(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    new_settings: SettingsArgs,
) -> ProgramResult {
    let account_iter = &mut accounts.iter();

    let authority_info = next_account_info(account_iter)?;
    let settings_info = next_account_info(account_iter)?;
    let old_settings = get_settings_checked(program_id, authority_info, settings_info)?;

    assert_authority(&old_settings, authority_info)?;

    MeepSettings {
        authority: *authority_info.key,
        secondary_creator: old_settings.secondary_creator,
        primary_wallet_percentage: new_settings.primary_wallet_percentage,
        minimum_price: new_settings.minimum_price,
    }
    .serialize(&mut *settings_info.data.borrow_mut())?;

    Ok(())
}
