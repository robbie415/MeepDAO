use crate::common::get_random_settings;
use common::rpc_client::NounsRpcClient;
use solana_sdk::{signature::Keypair, signer::Signer};

#[allow(dead_code)]
mod common;

#[test]
fn initialization() {
    let client = NounsRpcClient::new();

    for _ in 0..10 {
        let authority = Keypair::new();
        let secondary_creator = Keypair::new();
        let args = get_random_settings();

        let lamports = 1_000_000_000;
        client.airdrop(&authority, lamports);

        client
            .initialize_nouns(&authority, &secondary_creator, &args)
            .unwrap();

        let another_secondary_creator = Keypair::new();
        let another_args = get_random_settings();

        // double initialization
        assert!(client
            .initialize_nouns(&authority, &another_secondary_creator, &another_args)
            .is_err());

        let settings = client.get_settings(&authority.pubkey());
        assert_eq!(settings.authority, authority.pubkey());
        assert_eq!(settings.secondary_creator, secondary_creator.pubkey());
        assert_eq!(
            settings.primary_wallet_percentage,
            args.primary_wallet_percentage
        );
        assert_eq!(settings.minimum_price, args.minimum_price);
    }
}
