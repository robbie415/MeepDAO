use metaplex_token_metadata::state::{EDITION, PREFIX};
use solana_program::pubkey::Pubkey;

pub struct Pda;

impl Pda {
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
