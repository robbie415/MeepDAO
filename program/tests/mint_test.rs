use common::rpc_client::NounsRpcClient;
use nouns::{instruction::MintNftArgs, utils::Pda};
use rand::{thread_rng, Rng};
use solana_program::{pubkey::Pubkey, system_program, sysvar::rent};
use solana_sdk::{signature::Keypair, signer::Signer};

use crate::common::rpc_client::MintRawArguments;

mod common;

fn get_random_args() -> MintNftArgs {
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

#[test]
fn mint_token() {
    let client = NounsRpcClient::new();
    let creator = Keypair::new();
    let lamports = 1_000_000_000;
    client.airdrop(&creator, lamports);

    assert!(client.get_balance(&creator.pubkey()) >= lamports);

    for i in 1..=10 {
        let mint = Keypair::new();
        let token = Keypair::new();

        println!("Mint {}: {}", i, mint.pubkey());

        let args = get_random_args();
        client.mint_nft(&creator, &mint, &token, &args).unwrap();

        let metadata = client.get_metadata(&mint.pubkey());
        assert_eq!(metadata.mint, mint.pubkey());
        assert_eq!(metadata.update_authority, creator.pubkey());
        assert_eq!(metadata.primary_sale_happened, false);
        assert_eq!(metadata.is_mutable, false);

        let data = metadata.data;
        let creators = data.creators.unwrap();
        assert_eq!(data.name, args.token_name);
        assert_eq!(data.symbol, args.token_symbol);
        assert_eq!(data.uri, args.uri);
        assert_eq!(data.seller_fee_basis_points, args.seller_fee_basis_points);
        assert_eq!(creators.len(), 1);
        assert_eq!(creators[0].address, creator.pubkey());
        assert_eq!(creators[0].verified, true);
        assert_eq!(creators[0].share, 100);

        let edition = client.get_master_edition(&mint.pubkey());
        assert_eq!(edition.supply, 0);
        assert_eq!(edition.max_supply, Some(0));
    }
}

#[test]
fn mint_with_repeated_accounts() {
    let client = NounsRpcClient::new();
    let creator = Keypair::new();
    let lamports = 1_000_000_000;
    client.airdrop(&creator, lamports);

    assert!(client.get_balance(&creator.pubkey()) >= lamports);

    let mint = Keypair::new();
    let token = Keypair::new();

    let args = get_random_args();
    assert!(client.mint_nft(&creator, &mint, &token, &args).is_ok());
    assert!(client.mint_nft(&creator, &mint, &token, &args).is_err());

    let new_mint = Keypair::new();
    assert!(client.mint_nft(&creator, &new_mint, &token, &args).is_err());

    let new_token = Keypair::new();
    assert!(client.mint_nft(&creator, &mint, &new_token, &args).is_err());
    assert!(client
        .mint_nft(&creator, &new_mint, &new_token, &args)
        .is_ok());
}

#[test]
fn mint_with_wrong_accounts() {
    let client = NounsRpcClient::new();
    let creator = Keypair::new();
    let lamports = 1_000_000_000;
    client.airdrop(&creator, lamports);

    assert!(client.get_balance(&creator.pubkey()) >= lamports);

    let mint = Keypair::new();
    let token_account = Keypair::new();
    let metadata = Pda::metadata_pubkey(&mint.pubkey());
    let edition = Pda::master_edition_pubkey(&mint.pubkey());
    let system_program = system_program::ID;
    let token_program = spl_token::ID;
    let rent_program = rent::ID;
    let metaplex_program = metaplex_token_metadata::ID;
    let mint_args = get_random_args();

    let corrent_mint_args = MintRawArguments {
        creator,
        mint,
        token_account,
        metadata,
        edition,
        system_program,
        token_program,
        rent_program,
        metaplex_program,
        mint_args,
    };

    let wrong_metadata = MintRawArguments {
        metadata: Pubkey::new_unique(),
        ..corrent_mint_args.clone()
    };
    assert!(client.mint_nft_raw(&wrong_metadata).is_err());

    let wrong_edition = MintRawArguments {
        edition: Pubkey::new_unique(),
        ..corrent_mint_args.clone()
    };
    assert!(client.mint_nft_raw(&wrong_edition).is_err());

    let wrong_system_program = MintRawArguments {
        system_program: Pubkey::new_unique(),
        ..corrent_mint_args.clone()
    };
    assert!(client.mint_nft_raw(&wrong_system_program).is_err());

    let wrong_token_program = MintRawArguments {
        token_program: Pubkey::new_unique(),
        ..corrent_mint_args.clone()
    };
    assert!(client.mint_nft_raw(&wrong_token_program).is_err());

    let wrong_rent_program = MintRawArguments {
        rent_program: Pubkey::new_unique(),
        ..corrent_mint_args.clone()
    };
    assert!(client.mint_nft_raw(&wrong_rent_program).is_err());

    let wrong_metaplex_program = MintRawArguments {
        metaplex_program: Pubkey::new_unique(),
        ..corrent_mint_args.clone()
    };
    assert!(client.mint_nft_raw(&wrong_metaplex_program).is_err());

    // Correct
    assert!(client.mint_nft_raw(&corrent_mint_args).is_ok());
}
