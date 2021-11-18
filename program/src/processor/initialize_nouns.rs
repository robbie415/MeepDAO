use crate::{
    error::NounsError,
    instruction::SettingsArgs,
    state::NounsSettings,
    utils::{Pda, SETTINGS_SEED},
};
use borsh::BorshSerialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::invoke_signed,
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    sysvar::Sysvar,
};
use std::convert::TryInto;

pub fn create_settings_account<'info>(
    authority_info: &AccountInfo<'info>,
    settings_info: &AccountInfo<'info>,
    system_program: &AccountInfo<'info>,
    rent_program: &AccountInfo<'info>,
    program_id: &Pubkey,
    settings: &NounsSettings,
) -> ProgramResult {
    let rent = Rent::from_account_info(rent_program)?;
    let space = settings.try_to_vec()?.len();
    let lamports = rent.minimum_balance(space);

    let (settings_pubkey, bump) = Pda::settings_pubkey_with_bump(program_id, authority_info.key);
    let seeds = &[
        SETTINGS_SEED.as_bytes(),
        authority_info.key.as_ref(),
        &[bump],
    ];

    msg!("Create settings account");
    invoke_signed(
        &system_instruction::create_account(
            authority_info.key,
            &settings_pubkey,
            lamports,
            space.try_into().unwrap(),
            program_id,
        ),
        &[
            authority_info.clone(),
            settings_info.clone(),
            system_program.clone(),
        ],
        &[seeds],
    )?;

    settings.serialize(&mut *settings_info.data.borrow_mut())?;

    Ok(())
}

pub fn process_initialize(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    settings_args: SettingsArgs,
) -> ProgramResult {
    let account_iter = &mut accounts.iter();

    let authority_info = next_account_info(account_iter)?;
    let secondary_creator_info = next_account_info(account_iter)?;
    let settings_info = next_account_info(account_iter)?;
    let system_program = next_account_info(account_iter)?;
    let rent_program = next_account_info(account_iter)?;

    if !authority_info.is_signer || !secondary_creator_info.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    if authority_info.key == secondary_creator_info.key {
        return Err(NounsError::PrimareAndSecondaryAreSame.into());
    }

    if settings_args.primary_wallet_percentage > 100 {
        return Err(NounsError::PercentageLimitExceeded.into());
    }

    let settings = NounsSettings {
        authority: *authority_info.key,
        secondary_creator: *secondary_creator_info.key,
        primary_wallet_percentage: settings_args.primary_wallet_percentage,
        minimum_price: settings_args.minimum_price,
    };

    create_settings_account(
        authority_info,
        settings_info,
        system_program,
        rent_program,
        program_id,
        &settings,
    )
}
