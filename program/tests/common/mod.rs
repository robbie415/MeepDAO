use nouns::instruction::{MintNftArgs, SettingsArgs};
use rand::{thread_rng, Rng};
use std::{thread::sleep, time::Duration};

#[allow(dead_code)]
pub mod rpc_client;

pub fn delay() {
    sleep(Duration::new(0, 500_000_000));
}

pub fn get_random_mint_args() -> MintNftArgs {
    let mut rng = thread_rng();
    let seller_fee_basis_points = rng.gen_range(0..=10000);
    let token_name = format!("Token_name_{}", rng.gen_range(1..=100));
    let token_symbol = format!("TS_{}", rng.gen_range(1..=100));
    let uri = format!("https://test.com/{}", rng.gen_range(1..=100));

    MintNftArgs {
        seller_fee_basis_points,
        token_name: token_name.clone(),
        token_symbol: token_symbol.clone(),
        uri: uri.clone(),
    }
}

pub fn get_random_settings() -> SettingsArgs {
    let mut rng = thread_rng();
    SettingsArgs {
        primary_wallet_percentage: rng.gen_range(0..=100),
        minimum_price: rng.gen_range(0..=1_000_000_000),
    }
}
