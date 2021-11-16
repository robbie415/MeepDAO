use crate::{instruction::MintNftArgs, utils::Pda};
use metaplex_token_metadata::{
    instruction::{create_master_edition, create_metadata_accounts},
    state::Creator,
};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::invoke,
    program_pack::Pack,
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
    creator_info: &AccountInfo<'info>,
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
            creator_info.key,
            mint_info.key,
            lamports,
            space.try_into().unwrap(),
            &spl_token::ID,
        ),
        &[
            creator_info.clone(),
            mint_info.clone(),
            system_program.clone(),
        ],
    )?;

    msg!("Initialize mint");
    invoke(
        &initialize_mint(&spl_token::ID, mint_info.key, creator_info.key, None, 0)?,
        &[
            mint_info.clone(),
            creator_info.clone(),
            token_program.clone(),
            rent_program.clone(),
        ],
    )
}

fn prepare_token_account<'info>(
    creator_info: &AccountInfo<'info>,
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
            creator_info.key,
            token_account_info.key,
            lamports,
            space.try_into().unwrap(),
            &spl_token::ID,
        ),
        &[
            creator_info.clone(),
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
            creator_info.key,
        )?,
        &[
            creator_info.clone(),
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
            creator_info.key,
            &[creator_info.key],
            1,
        )?,
        &[
            creator_info.clone(),
            token_account_info.clone(),
            mint_info.clone(),
            token_program.clone(),
        ],
    )
}

fn init_metadata<'info>(
    creator_info: &AccountInfo<'info>,
    mint_info: &AccountInfo<'info>,
    token_metadata_info: &AccountInfo<'info>,
    system_program: &AccountInfo<'info>,
    rent_program: &AccountInfo<'info>,
    metaplex_program: &AccountInfo<'info>,
    mint_args: MintNftArgs,
) -> ProgramResult {
    let metadata_pubkey = Pda::metadata_pubkey(mint_info.key);

    let creator = Creator {
        address: *creator_info.key,
        share: 100,
        verified: true,
    };

    msg!("Create metadata account");
    invoke(
        &create_metadata_accounts(
            metaplex_token_metadata::ID,
            metadata_pubkey,
            *mint_info.key,
            *creator_info.key,
            *creator_info.key,
            *creator_info.key,
            mint_args.token_name,
            mint_args.token_symbol,
            mint_args.uri,
            Some(vec![creator]),
            mint_args.seller_fee_basis_points,
            true,
            false,
        ),
        &[
            creator_info.clone(),
            token_metadata_info.clone(),
            mint_info.clone(),
            system_program.clone(),
            rent_program.clone(),
            metaplex_program.clone(),
        ],
    )
}

fn init_master_edition<'info>(
    creator_info: &AccountInfo<'info>,
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
            *creator_info.key,
            *creator_info.key,
            *token_metadata_info.key,
            *creator_info.key,
            Some(0),
        ),
        &[
            creator_info.clone(),
            mint_info.clone(),
            token_metadata_info.clone(),
            master_edition_info.clone(),
            system_program.clone(),
            metaplex_program.clone(),
            rent_program.clone(),
        ],
    )
}

pub fn process_mint(accounts: &[AccountInfo], mint_args: MintNftArgs) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let creator_info = next_account_info(accounts_iter)?;
    let mint_info = next_account_info(accounts_iter)?;
    let token_account_info = next_account_info(accounts_iter)?;
    let token_metadata_info = next_account_info(accounts_iter)?;
    let master_edition_info = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;
    let token_program = next_account_info(accounts_iter)?;
    let rent_program = next_account_info(accounts_iter)?;
    let metaplex_program = next_account_info(accounts_iter)?;

    prepare_mint_account(
        creator_info,
        mint_info,
        system_program,
        token_program,
        rent_program,
    )?;

    prepare_token_account(
        creator_info,
        token_account_info,
        mint_info,
        system_program,
        token_program,
        rent_program,
    )?;

    init_metadata(
        creator_info,
        mint_info,
        token_metadata_info,
        system_program,
        rent_program,
        metaplex_program,
        mint_args,
    )?;

    init_master_edition(
        creator_info,
        mint_info,
        token_metadata_info,
        master_edition_info,
        system_program,
        rent_program,
        metaplex_program,
    )
}
