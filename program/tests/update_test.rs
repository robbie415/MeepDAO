use common::{get_random_settings, rpc_client::MeepRpcClient};
use solana_sdk::{signature::Keypair, signer::Signer};

#[allow(dead_code)]
mod common;

#[test]
fn update_test() {
    let client = MeepRpcClient::new();

    for _ in 0..10 {
        let authority = Keypair::new();
        let secondary_creator = Keypair::new();
        let settings = get_random_settings();

        let lamports = 1_000_000_000;
        client.airdrop(&authority, lamports);

        assert!(client.update_settings(&authority, &settings).is_err());

        client
            .initialize_meep(&authority, &secondary_creator, &settings)
            .unwrap();

        let new_settings = get_random_settings();

        client.update_settings(&authority, &new_settings).unwrap();

        let on_chain_settings = client.get_settings(&authority.pubkey());

        assert_eq!(on_chain_settings.authority, authority.pubkey());
        assert_eq!(
            on_chain_settings.secondary_creator,
            secondary_creator.pubkey()
        );
        assert_eq!(on_chain_settings.minimum_price, new_settings.minimum_price);
        assert_eq!(
            on_chain_settings.primary_wallet_percentage,
            new_settings.primary_wallet_percentage
        );
    }
}
