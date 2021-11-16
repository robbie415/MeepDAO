pub mod error;
pub mod instruction;
pub mod processor;
pub mod state;
pub mod utils;

#[cfg(not(feature = "no-entrypoint"))]
pub mod entrypoint;

solana_program::declare_id!("5Hu2bnTxd1mPXNHqMzFfB5SUFEvYW7GG3nPSQ1VWvTK");
