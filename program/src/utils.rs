use crate::{error::MeepError, state::MeepSettings};
use borsh::BorshDeserialize;
use metaplex_token_metadata::state::{EDITION, PREFIX};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
    pubkey::Pubkey,
};

pub const SETTINGS_SEED: &str = "settings_meep";

pub struct Pda;

impl Pda {
    pub fn settings_pubkey_with_bump(program_id: &Pubkey, authority: &Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(&[SETTINGS_SEED.as_bytes(), authority.as_ref()], program_id)
    }

    pub fn metadata_pubkey(mint: &Pubkey) -> Pubkey {
        let seeds = &[
            PREFIX.as_bytes(),
            metaplex_token_metadata::ID.as_ref(),
            mint.as_ref(),
        ];

        Pubkey::find_program_address(seeds, &metaplex_token_metadata::ID).0
    }

    pub fn master_edition_pubkey(mint: &Pubkey) -> Pubkey {
        let seeds = &[
            PREFIX.as_bytes(),
            metaplex_token_metadata::ID.as_ref(),
            mint.as_ref(),
            EDITION.as_bytes(),
        ];

        Pubkey::find_program_address(seeds, &metaplex_token_metadata::ID).0
    }
}

pub fn get_settings_checked<'info>(
    program_id: &Pubkey,
    authority_info: &AccountInfo<'info>,
    settings_info: &AccountInfo<'info>,
) -> Result<MeepSettings, ProgramError> {
    assert_settings(program_id, authority_info, settings_info)?;
    let settings = MeepSettings::try_from_slice(&settings_info.data.borrow());
    settings.map_err(|_| MeepError::WrongSettingsAccount.into())
}

pub fn assert_settings(
    program_id: &Pubkey,
    authority_info: &AccountInfo,
    settings_info: &AccountInfo,
) -> ProgramResult {
    let settings_pubkey = Pda::settings_pubkey_with_bump(program_id, authority_info.key).0;
    if *settings_info.key != settings_pubkey {
        return Err(MeepError::WrongSettingsAccount.into());
    }

    Ok(())
}

pub fn assert_authority(settings: &MeepSettings, authority_info: &AccountInfo) -> ProgramResult {
    if !authority_info.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    if *authority_info.key != settings.authority {
        return Err(MeepError::WrongAuthority.into());
    }

    Ok(())
}

pub fn assert_secondary_creator(
    settings: &MeepSettings,
    secondary_creator_info: &AccountInfo,
) -> ProgramResult {
    if *secondary_creator_info.key != settings.secondary_creator {
        return Err(MeepError::WrongSecondaryCreator.into());
    }

    Ok(())
}
