use common::{get_random_mint_args, get_random_settings, rpc_client::MeepRpcClient};
use solana_sdk::{signature::Keypair, signer::Signer};

mod common;

#[test]
fn mint_token() {
    let client = MeepRpcClient::new();
    let lamports = 1_000_000_000;

    for i in 1..=10 {
        let authority = Keypair::new();
        let secondary_creator = Keypair::new();
        let mint = Keypair::new();
        let token = Keypair::new();
        let initialize_args = get_random_settings();
        let mint_args = get_random_mint_args();

        println!("Mint {}: {}", i, mint.pubkey());

        client.airdrop(&authority, lamports);

        let authority_initial_balance = client.get_balance(&authority.pubkey());

        client
            .initialize_meep(&authority, &secondary_creator, &initialize_args)
            .unwrap();

        let authority_balance = client.get_balance(&authority.pubkey());

        client
            .mint_nft(&authority, &secondary_creator, &mint, &token, &mint_args)
            .unwrap();

        println!(
            "Initialization cost for authority: {} lamports",
            authority_initial_balance - authority_balance
        );
        println!(
            "Minting cost for authority: {} lamports",
            authority_balance - client.get_balance(&authority.pubkey())
        );

        let metadata = client.get_metadata(&mint.pubkey());
        assert_eq!(metadata.mint, mint.pubkey());
        assert_eq!(metadata.update_authority, authority.pubkey());
        assert_eq!(metadata.primary_sale_happened, false);
        assert_eq!(metadata.is_mutable, false);

        let data = metadata.data;
        let creators = data.creators.unwrap();
        assert_eq!(data.name, mint_args.token_name);
        assert_eq!(data.symbol, mint_args.token_symbol);
        assert_eq!(data.uri, mint_args.uri);
        assert_eq!(
            data.seller_fee_basis_points,
            mint_args.seller_fee_basis_points
        );

        assert_eq!(creators.len(), 2);
        assert_eq!(creators[0].address, authority.pubkey());
        assert_eq!(creators[0].verified, true);
        assert_eq!(creators[0].share, initialize_args.primary_wallet_percentage);

        assert_eq!(creators[1].address, secondary_creator.pubkey());
        assert_eq!(creators[1].verified, true);
        assert_eq!(
            creators[1].share,
            100 - initialize_args.primary_wallet_percentage
        );

        let edition = client.get_master_edition(&mint.pubkey());
        assert_eq!(edition.supply, 0);
        assert_eq!(edition.max_supply, Some(0));
    }
}
