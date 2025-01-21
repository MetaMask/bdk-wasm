use bdk_wallet::{SignOptions, Wallet as BdkWallet};
use js_sys::Date;
use serde_wasm_bindgen::to_value;
use wasm_bindgen::{prelude::wasm_bindgen, JsError, JsValue};

use crate::{
    result::JsResult,
    types::{
        AddressInfo, Balance, ChangeSet, CheckPoint, FeeRate, FullScanRequest, KeychainKind, Network, Psbt, Recipient,
        SyncRequest, Update,
    },
};

#[wasm_bindgen]
pub struct Wallet(BdkWallet);

#[wasm_bindgen]
impl Wallet {
    pub fn create(network: Network, external_descriptor: String, internal_descriptor: String) -> JsResult<Wallet> {
        let wallet = BdkWallet::create(external_descriptor, internal_descriptor)
            .network(network.into())
            .create_wallet_no_persist()?;

        Ok(Wallet(wallet))
    }

    pub fn load(
        changeset: ChangeSet,
        external_descriptor: Option<String>,
        internal_descriptor: Option<String>,
    ) -> JsResult<Wallet> {
        let mut builder = BdkWallet::load();

        if external_descriptor.is_some() {
            builder = builder.descriptor(KeychainKind::External.into(), external_descriptor);
        }

        if internal_descriptor.is_some() {
            builder = builder.descriptor(KeychainKind::Internal.into(), internal_descriptor);
        }

        let wallet_opt = builder.extract_keys().load_wallet_no_persist(changeset.into())?;

        let wallet = match wallet_opt {
            Some(wallet) => wallet,
            None => return Err(JsError::new("Failed to load wallet, check the changeset")),
        };

        Ok(Wallet(wallet))
    }

    pub fn start_full_scan(&self) -> FullScanRequest {
        self.0.start_full_scan().build().into()
    }

    pub fn start_sync_with_revealed_spks(&self) -> SyncRequest {
        self.0.start_sync_with_revealed_spks().build().into()
    }

    pub fn apply_update(&mut self, update: Update) -> JsResult<()> {
        self.apply_update_at(update, (Date::now() / 1000.0) as u64)
    }

    pub fn apply_update_at(&mut self, update: Update, seen_at: u64) -> JsResult<()> {
        self.0.apply_update_at(update, seen_at)?;
        Ok(())
    }

    pub fn network(&self) -> Network {
        self.0.network().into()
    }

    pub fn balance(&self) -> Balance {
        self.0.balance().into()
    }

    pub fn next_unused_address(&mut self, keychain: KeychainKind) -> AddressInfo {
        self.0.next_unused_address(keychain.into()).into()
    }

    pub fn peek_address(&self, keychain: KeychainKind, index: u32) -> AddressInfo {
        self.0.peek_address(keychain.into(), index).into()
    }

    pub fn reveal_next_address(&mut self, keychain: KeychainKind) -> AddressInfo {
        self.0.reveal_next_address(keychain.into()).into()
    }

    pub fn reveal_addresses_to(&mut self, keychain: KeychainKind, index: u32) -> Vec<AddressInfo> {
        self.0
            .reveal_addresses_to(keychain.into(), index)
            .map(Into::into)
            .collect()
    }

    pub fn list_unused_addresses(&self, keychain: KeychainKind) -> Vec<AddressInfo> {
        self.0.list_unused_addresses(keychain.into()).map(Into::into).collect()
    }

    pub fn list_unspent(&self) -> JsResult<Vec<JsValue>> {
        self.0
            .list_unspent()
            .map(|output| to_value(&output).map_err(Into::into))
            .collect()
    }

    pub fn transactions(&self) -> JsResult<Vec<JsValue>> {
        self.0
            .transactions()
            .map(|tx| to_value(&tx.tx_node.tx).map_err(Into::into))
            .collect()
    }

    pub fn latest_checkpoint(&self) -> CheckPoint {
        self.0.latest_checkpoint().into()
    }

    pub fn take_staged(&mut self) -> Option<ChangeSet> {
        self.0.take_staged().map(Into::into)
    }

    pub fn public_descriptor(&self, keychain: KeychainKind) -> String {
        self.0.public_descriptor(keychain.into()).to_string()
    }

    pub fn build_tx(&mut self, fee_rate: FeeRate, recipients: Vec<Recipient>) -> JsResult<Psbt> {
        let mut builder = self.0.build_tx();

        builder
            .set_recipients(recipients.into_iter().map(Into::into).collect())
            .fee_rate(fee_rate.into());

        let psbt = builder.finish()?;
        Ok(psbt.into())
    }

    pub fn sign(&self, psbt: &mut Psbt) -> JsResult<bool> {
        let result = self.0.sign(psbt, SignOptions::default())?;
        Ok(result)
    }
}
