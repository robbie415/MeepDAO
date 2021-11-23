use crate::{
    instruction::MintNftArgs,
    state::MeepSettings,
    utils::{assert_authority, assert_secondary_creator, get_settings_checked, Pda},
};
use metaplex_token_metadata::{
    instruction::{create_master_edition, create_metadata_accounts, sign_metadata},
    state::Creator,
};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::invoke,
    program_pack::Pack,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    sysvar::Sysvar,
};
use spl_token::{
    instruction::{initialize_account, initialize_mint, mint_to},
    state::{Account, Mint},
};
use std::convert::TryInto;

fn prepare_mint_account<'info>(
    authority_info: &AccountInfo<'info>,
    mint_info: &AccountInfo<'info>,
    system_program: &AccountInfo<'info>,
    token_program: &AccountInfo<'info>,
    rent_program: &AccountInfo<'info>,
) -> ProgramResult {
    let rent = Rent::from_account_info(rent_program)?;
    let space = Mint::LEN;
    let lamports = rent.minimum_balance(space);

    msg!("Create account for mint");
    invoke(
        &system_instruction::create_account(
            authority_info.key,
            mint_info.key,
            lamports,
            space.try_into().unwrap(),
            &spl_token::ID,
        ),
        &[
            authority_info.clone(),
            mint_info.clone(),
            system_program.clone(),
        ],
    )?;

    msg!("Initialize mint");
    invoke(
        &initialize_mint(&spl_token::ID, mint_info.key, authority_info.key, None, 0)?,
        &[
            mint_info.clone(),
            authority_info.clone(),
            token_program.clone(),
            rent_program.clone(),
        ],
    )
}

fn prepare_token_account<'info>(
    authority_info: &AccountInfo<'info>,
    token_account_info: &AccountInfo<'info>,
    mint_info: &AccountInfo<'info>,
    system_program: &AccountInfo<'info>,
    token_program: &AccountInfo<'info>,
    rent_program: &AccountInfo<'info>,
) -> ProgramResult {
    let rent = Rent::from_account_info(rent_program)?;
    let space = Account::LEN;
    let lamports = rent.minimum_balance(space);

    msg!("Create token account");
    invoke(
        &system_instruction::create_account(
            authority_info.key,
            token_account_info.key,
            lamports,
            space.try_into().unwrap(),
            &spl_token::ID,
        ),
        &[
            authority_info.clone(),
            token_account_info.clone(),
            system_program.clone(),
        ],
    )?;

    msg!("Initialize token account");
    invoke(
        &initialize_account(
            &spl_token::ID,
            token_account_info.key,
            mint_info.key,
            authority_info.key,
        )?,
        &[
            authority_info.clone(),
            token_account_info.clone(),
            mint_info.clone(),
            token_program.clone(),
            rent_program.clone(),
        ],
    )?;

    msg!("Mint one token");
    invoke(
        &mint_to(
            &spl_token::ID,
            mint_info.key,
            token_account_info.key,
            authority_info.key,
            &[authority_info.key],
            1,
        )?,
        &[
            authority_info.clone(),
            token_account_info.clone(),
            mint_info.clone(),
            token_program.clone(),
        ],
    )
}

#[allow(clippy::too_many_arguments)]
fn init_metadata<'info>(
    authority_info: &AccountInfo<'info>,
    secondary_creator_info: &AccountInfo<'info>,
    mint_info: &AccountInfo<'info>,
    token_metadata_info: &AccountInfo<'info>,
    system_program: &AccountInfo<'info>,
    rent_program: &AccountInfo<'info>,
    metaplex_program: &AccountInfo<'info>,
    settings: &MeepSettings,
    mint_args: MintNftArgs,
) -> ProgramResult {
    let metadata_pubkey = Pda::metadata_pubkey(mint_info.key);

    let primary_creator = Creator {
        address: settings.authority,
        share: settings.primary_wallet_percentage,
        verified: true,
    };

    let secondary_creator = Creator {
        address: settings.secondary_creator,
        share: 100 - settings.primary_wallet_percentage,
        verified: false,
    };

    let creators = Some(vec![primary_creator, secondary_creator]);

    msg!("Create metadata account");
    invoke(
        &create_metadata_accounts(
            metaplex_token_metadata::ID,
            metadata_pubkey,
            *mint_info.key,
            *authority_info.key,
            *authority_info.key,
            *authority_info.key,
            mint_args.token_name,
            mint_args.token_symbol,
            mint_args.uri,
            creators,
            mint_args.seller_fee_basis_points,
            true,
            false,
        ),
        &[
            authority_info.clone(),
            secondary_creator_info.clone(),
            token_metadata_info.clone(),
            mint_info.clone(),
            system_program.clone(),
            rent_program.clone(),
            metaplex_program.clone(),
        ],
    )?;

    invoke(
        &sign_metadata(
            metaplex_token_metadata::ID,
            metadata_pubkey,
            *secondary_creator_info.key,
        ),
        &[
            token_metadata_info.clone(),
            secondary_creator_info.clone(),
            metaplex_program.clone(),
        ],
    )
}

fn init_master_edition<'info>(
    authority_info: &AccountInfo<'info>,
    mint_info: &AccountInfo<'info>,
    token_metadata_info: &AccountInfo<'info>,
    master_edition_info: &AccountInfo<'info>,
    system_program: &AccountInfo<'info>,
    rent_program: &AccountInfo<'info>,
    metaplex_program: &AccountInfo<'info>,
) -> ProgramResult {
    let edition_pubkey = Pda::master_edition_pubkey(mint_info.key);

    msg!("Create master edition account");
    invoke(
        &create_master_edition(
            metaplex_token_metadata::ID,
            edition_pubkey,
            *mint_info.key,
            *authority_info.key,
            *authority_info.key,
            *token_metadata_info.key,
            *authority_info.key,
            Some(0),
        ),
        &[
            authority_info.clone(),
            mint_info.clone(),
            token_metadata_info.clone(),
            master_edition_info.clone(),
            system_program.clone(),
            metaplex_program.clone(),
            rent_program.clone(),
        ],
    )
}

pub fn process_mint(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    mint_args: MintNftArgs,
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let authority_info = next_account_info(accounts_iter)?;
    let secondary_creator_info = next_account_info(accounts_iter)?;
    let settings_info = next_account_info(accounts_iter)?;
    let mint_info = next_account_info(accounts_iter)?;
    let token_account_info = next_account_info(accounts_iter)?;
    let token_metadata_info = next_account_info(accounts_iter)?;
    let master_edition_info = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;
    let token_program = next_account_info(accounts_iter)?;
    let rent_program = next_account_info(accounts_iter)?;
    let metaplex_program = next_account_info(accounts_iter)?;

    let settings = get_settings_checked(program_id, authority_info, settings_info)?;

    assert_authority(&settings, authority_info)?;
    assert_secondary_creator(&settings, secondary_creator_info)?;

    prepare_mint_account(
        authority_info,
        mint_info,
        system_program,
        token_program,
        rent_program,
    )?;

    prepare_token_account(
        authority_info,
        token_account_info,
        mint_info,
        system_program,
        token_program,
        rent_program,
    )?;

    init_metadata(
        authority_info,
        secondary_creator_info,
        mint_info,
        token_metadata_info,
        system_program,
        rent_program,
        metaplex_program,
        &settings,
        mint_args,
    )?;

    init_master_edition(
        authority_info,
        mint_info,
        token_metadata_info,
        master_edition_info,
        system_program,
        rent_program,
        metaplex_program,
    )
}
