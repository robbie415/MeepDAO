use num_derive::FromPrimitive;
use solana_program::{
    decode_error::DecodeError,
    msg,
    program_error::{PrintProgramError, ProgramError},
};
use thiserror::Error;

#[derive(Clone, Debug, Error, FromPrimitive)]
pub enum MeepError {
    #[error("Primary and secondary creators must be different")]
    PrimareAndSecondaryAreSame,

    #[error("Percentage is in the range [0; 100]")]
    PercentageLimitExceeded,

    #[error("Settings account has wrong pubkey")]
    WrongSettingsAccount,

    #[error("Wrong authority")]
    WrongAuthority,

    #[error("Wrong primary creator")]
    WrongPrimaryCreator,

    #[error("Wrong secondary creator")]
    WrongSecondaryCreator,
}

impl From<MeepError> for ProgramError {
    fn from(error: MeepError) -> Self {
        ProgramError::Custom(error as u32)
    }
}

impl PrintProgramError for MeepError {
    fn print<E>(&self) {
        msg!(&self.to_string());
    }
}

impl<T> DecodeError<T> for MeepError {
    fn type_of() -> &'static str {
        "Meep Error"
    }
}
