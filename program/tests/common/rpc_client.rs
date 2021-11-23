use super::delay;
use borsh::BorshDeserialize;
use meep::{
    instruction::{MeepInstructions, MintNftArgs, SettingsArgs},
    state::MeepSettings,
    utils::Pda,
};
use metaplex_token_metadata::state::{MasterEditionV2, Metadata};
use solana_client::{client_error::ClientError, rpc_client::RpcClient};
use solana_program::{borsh::try_from_slice_unchecked, pubkey::Pubkey, system_program};
use solana_sdk::{
    commitment_config::CommitmentConfig,
    signature::{Keypair, Signature},
    signer::Signer,
    system_transaction,
    transaction::Transaction,
};
use std::time::Duration;

pub struct MeepRpcClient {
    client: RpcClient,
    fee_payer: Keypair,
    program_id: Pubkey,
}

impl MeepRpcClient {
    pub fn new() -> MeepRpcClient {
        MeepRpcClient::new_with_program_id(&meep::ID)
    }

    pub fn new_with_program_id(program_id: &Pubkey) -> MeepRpcClient {
        let uri = "http://localhost:8899".to_string();
        let client = RpcClient::new_with_timeout_and_commitment(
            uri,
            Duration::new(300, 0),
            CommitmentConfig::confirmed(),
        );

        let client = MeepRpcClient {
            client,
            fee_payer: Keypair::new(),
            program_id: *program_id,
        };

        client.airdrop(&client.fee_payer, 1_000_000_000);
        client
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

    pub fn initialize_meep(
        &self,
        authority: &Keypair,
        secondary_creator: &Keypair,
        initialize_args: &SettingsArgs,
    ) -> Result<Signature, ClientError> {
        let ix = MeepInstructions::initialize_meep(
            &self.program_id,
            &authority.pubkey(),
            &secondary_creator.pubkey(),
            initialize_args,
        );

        let blockhash = self.client.get_recent_blockhash().unwrap().0;
        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&self.fee_payer.pubkey()),
            &vec![authority, &self.fee_payer, secondary_creator],
            blockhash,
        );

        self.client.send_and_confirm_transaction_with_spinner(&tx)
    }

    pub fn update_settings(
        &self,
        authority: &Keypair,
        settings: &SettingsArgs,
    ) -> Result<Signature, ClientError> {
        let ix = MeepInstructions::update_settings(&self.program_id, &authority.pubkey(), settings);

        let blockhash = self.client.get_recent_blockhash().unwrap().0;
        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&self.fee_payer.pubkey()),
            &[authority, &self.fee_payer],
            blockhash,
        );

        self.client.send_and_confirm_transaction_with_spinner(&tx)
    }

    pub fn mint_nft(
        &self,
        authority: &Keypair,
        secondary_creator: &Keypair,
        mint: &Keypair,
        token_account: &Keypair,
        mint_args: &MintNftArgs,
    ) -> Result<Signature, ClientError> {
        let ix = MeepInstructions::mint_nft(
            &self.program_id,
            &authority.pubkey(),
            &secondary_creator.pubkey(),
            &mint.pubkey(),
            &token_account.pubkey(),
            mint_args,
        );

        let blockhash = self.client.get_recent_blockhash().unwrap().0;
        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&self.fee_payer.pubkey()),
            &vec![
                authority,
                &self.fee_payer,
                secondary_creator,
                mint,
                token_account,
            ],
            blockhash,
        );

        self.client.send_and_confirm_transaction_with_spinner(&tx)
    }

    pub fn get_settings(&self, authority: &Pubkey) -> MeepSettings {
        let settings_pubkey = Pda::settings_pubkey_with_bump(&self.program_id, authority).0;
        let settings_data = self.client.get_account_data(&settings_pubkey).unwrap();
        MeepSettings::try_from_slice(&settings_data).unwrap()
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
