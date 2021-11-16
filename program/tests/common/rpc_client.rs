use super::delay;
use metaplex_token_metadata::state::{MasterEditionV2, Metadata};
use nouns::{
    instruction::{MintNftArgs, NounsInstructions},
    utils::Pda,
};
use solana_client::{client_error::ClientError, rpc_client::RpcClient};
use solana_program::{
    borsh::try_from_slice_unchecked,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_program,
};
use solana_sdk::{
    commitment_config::CommitmentConfig,
    signature::{Keypair, Signature},
    signer::Signer,
    system_transaction,
    transaction::Transaction,
};
use std::time::Duration;

pub struct NounsRpcClient {
    client: RpcClient,
    program_id: Pubkey,
}

pub struct MintRawArguments {
    pub creator: Keypair,
    pub mint: Keypair,
    pub token_account: Keypair,
    pub metadata: Pubkey,
    pub edition: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub rent_program: Pubkey,
    pub metaplex_program: Pubkey,
    pub mint_args: MintNftArgs,
}

impl Clone for MintRawArguments {
    fn clone(&self) -> Self {
        MintRawArguments {
            creator: Keypair::from_bytes(&self.creator.to_bytes()).unwrap(),
            mint: Keypair::from_bytes(&self.mint.to_bytes()).unwrap(),
            token_account: Keypair::from_bytes(&self.token_account.to_bytes()).unwrap(),
            metadata: self.metadata,
            edition: self.edition,
            system_program: self.system_program,
            token_program: self.token_program,
            rent_program: self.rent_program,
            metaplex_program: self.metaplex_program,
            mint_args: self.mint_args.clone(),
        }
    }
}

impl NounsRpcClient {
    pub fn new() -> NounsRpcClient {
        NounsRpcClient::new_with_program_id(&nouns::ID)
    }

    pub fn new_with_program_id(program_id: &Pubkey) -> NounsRpcClient {
        let uri = "http://localhost:8899".to_string();
        let client = RpcClient::new_with_timeout_and_commitment(
            uri,
            Duration::new(300, 0),
            CommitmentConfig::confirmed(),
        );

        NounsRpcClient {
            client,
            program_id: *program_id,
        }
    }

    pub fn airdrop(&self, wallet: &Keypair, lamports: u64) {
        let current_balance = self.client.get_balance(&wallet.pubkey()).unwrap();
        let (blockhash, fees) = self.client.get_recent_blockhash().unwrap();

        if current_balance > lamports {
            let tx = system_transaction::transfer(
                wallet,
                &system_program::ID,
                current_balance - lamports - fees.lamports_per_signature,
                blockhash,
            );

            self.client
                .send_and_confirm_transaction_with_spinner(&tx)
                .unwrap();
            return;
        }

        loop {
            let current_balance = self.get_balance(&wallet.pubkey());
            if current_balance >= lamports {
                return;
            }

            self.client
                .request_airdrop(&wallet.pubkey(), lamports - current_balance)
                .unwrap();

            delay();
        }
    }

    pub fn get_balance(&self, wallet: &Pubkey) -> u64 {
        self.client.get_balance(wallet).unwrap()
    }

    pub fn mint_nft_raw(&self, raw_args: &MintRawArguments) -> Result<Signature, ClientError> {
        let ix = Instruction::new_with_borsh(
            self.program_id,
            &NounsInstructions::MintNft(raw_args.mint_args.clone()),
            vec![
                AccountMeta::new(raw_args.creator.pubkey(), true),
                AccountMeta::new(raw_args.mint.pubkey(), true),
                AccountMeta::new(raw_args.token_account.pubkey(), true),
                AccountMeta::new(raw_args.metadata, false),
                AccountMeta::new(raw_args.edition, false),
                AccountMeta::new_readonly(raw_args.system_program, false),
                AccountMeta::new_readonly(raw_args.token_program, false),
                AccountMeta::new_readonly(raw_args.rent_program, false),
                AccountMeta::new_readonly(raw_args.metaplex_program, false),
            ],
        );

        let blockhash = self.client.get_recent_blockhash().unwrap().0;
        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&raw_args.creator.pubkey()),
            &[&raw_args.creator, &raw_args.mint, &raw_args.token_account],
            blockhash,
        );

        self.client.send_and_confirm_transaction_with_spinner(&tx)
    }

    pub fn mint_nft(
        &self,
        creator: &Keypair,
        mint: &Keypair,
        token_account: &Keypair,
        mint_args: &MintNftArgs,
    ) -> Result<Signature, ClientError> {
        let ix = NounsInstructions::mint_nft(
            &self.program_id,
            &creator.pubkey(),
            &mint.pubkey(),
            &token_account.pubkey(),
            mint_args,
        );

        let blockhash = self.client.get_recent_blockhash().unwrap().0;
        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&creator.pubkey()),
            &[creator, mint, token_account],
            blockhash,
        );

        self.client.send_and_confirm_transaction_with_spinner(&tx)
    }

    pub fn get_metadata(&self, mint: &Pubkey) -> Metadata {
        let metadata_pubkey = Pda::metadata_pubkey(mint);
        let metadata_data = self.client.get_account_data(&metadata_pubkey).unwrap();
        let mut metadata: Metadata = try_from_slice_unchecked(&metadata_data).unwrap();
        let zero = char::from(0);
        metadata.data.name = metadata.data.name.trim_end_matches(zero).to_string();
        metadata.data.symbol = metadata.data.symbol.trim_end_matches(zero).to_string();
        metadata.data.uri = metadata.data.uri.trim_end_matches(zero).to_string();
        metadata
    }

    pub fn get_master_edition(&self, mint: &Pubkey) -> MasterEditionV2 {
        let edition_pubkey = Pda::master_edition_pubkey(mint);
        let edition_data = self.client.get_account_data(&edition_pubkey).unwrap();
        try_from_slice_unchecked(&edition_data).unwrap()
    }
}
