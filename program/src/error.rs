use num_derive::FromPrimitive;
use solana_program::{
    decode_error::DecodeError,
    msg,
    program_error::{PrintProgramError, ProgramError},
};
use thiserror::Error;

#[derive(Clone, Debug, Error, FromPrimitive)]
pub enum NounsError {
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

impl From<NounsError> for ProgramError {
    fn from(error: NounsError) -> Self {
        ProgramError::Custom(error as u32)
    }
}

impl PrintProgramError for NounsError {
    fn print<E>(&self) {
        msg!(&self.to_string());
    }
}

impl<T> DecodeError<T> for NounsError {
    fn type_of() -> &'static str {
        "Nouns Error"
    }
}
