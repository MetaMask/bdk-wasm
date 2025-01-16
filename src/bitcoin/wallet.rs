use bdk_wallet::{SignOptions, Wallet as BdkWallet};
use js_sys::Date;
use serde_wasm_bindgen::to_value;
use wasm_bindgen::{prelude::wasm_bindgen, JsError, JsValue};

use crate::{
    result::JsResult,
    types::{
        AddressInfo, Balance, ChangeSet, CheckPoint, DescriptorPair, FeeRate, FullScanRequest, KeychainKind, Network,
        Psbt, Recipient, SyncRequest, Update,
    },
};

#[wasm_bindgen]
pub struct Wallet {
    wallet: BdkWallet,
}

#[wasm_bindgen]
impl Wallet {
    pub fn create(network: Network, descriptors: DescriptorPair) -> JsResult<Wallet> {
        let wallet = BdkWallet::create(descriptors.external(), descriptors.internal())
            .network(network.into())
            .create_wallet_no_persist()?;

        Ok(Wallet { wallet })
    }

    pub fn load(changeset: ChangeSet) -> JsResult<Wallet> {
        let wallet_opt = BdkWallet::load().load_wallet_no_persist(changeset.into())?;

        let wallet = match wallet_opt {
            Some(wallet) => wallet,
            None => return Err(JsError::new("Failed to load wallet, check the changeset")),
        };

        Ok(Wallet { wallet })
    }

    pub fn start_full_scan(&self) -> FullScanRequest {
        self.wallet.start_full_scan().build().into()
    }

    pub fn start_sync_with_revealed_spks(&self) -> SyncRequest {
        self.wallet.start_sync_with_revealed_spks().build().into()
    }

    pub fn apply_update(&mut self, update: Update) -> JsResult<()> {
        self.apply_update_at(update, (Date::now() / 1000.0) as u64)
    }

    pub fn apply_update_at(&mut self, update: Update, seen_at: u64) -> JsResult<()> {
        self.wallet.apply_update_at(update, seen_at)?;
        Ok(())
    }

    pub fn network(&self) -> Network {
        self.wallet.network().into()
    }

    pub fn balance(&self) -> Balance {
        self.wallet.balance().into()
    }

    pub fn next_unused_address(&mut self, keychain: KeychainKind) -> AddressInfo {
        self.wallet.next_unused_address(keychain.into()).into()
    }

    pub fn peek_address(&self, keychain: KeychainKind, index: u32) -> AddressInfo {
        self.wallet.peek_address(keychain.into(), index).into()
    }

    pub fn reveal_next_address(&mut self, keychain: KeychainKind) -> AddressInfo {
        self.wallet.reveal_next_address(keychain.into()).into()
    }

    pub fn reveal_addresses_to(&mut self, keychain: KeychainKind, index: u32) -> Vec<AddressInfo> {
        self.wallet
            .reveal_addresses_to(keychain.into(), index)
            .map(Into::into)
            .collect()
    }

    pub fn list_unused_addresses(&self, keychain: KeychainKind) -> Vec<AddressInfo> {
        self.wallet
            .list_unused_addresses(keychain.into())
            .map(Into::into)
            .collect()
    }

    pub fn list_unspent(&self) -> JsResult<Vec<JsValue>> {
        self.wallet
            .list_unspent()
            .map(|output| to_value(&output).map_err(Into::into))
            .collect()
    }

    pub fn transactions(&self) -> JsResult<Vec<JsValue>> {
        self.wallet
            .transactions()
            .map(|tx| to_value(&tx.tx_node.tx).map_err(Into::into))
            .collect()
    }

    pub fn latest_checkpoint(&self) -> CheckPoint {
        self.wallet.latest_checkpoint().into()
    }

    pub fn take_staged(&mut self) -> Option<ChangeSet> {
        self.wallet.take_staged().map(Into::into)
    }

    pub fn public_descriptor(&self, keychain: KeychainKind) -> String {
        self.wallet.public_descriptor(keychain.into()).to_string()
    }

    pub fn build_tx(&mut self, fee_rate: FeeRate, recipients: Vec<Recipient>) -> JsResult<Psbt> {
        let mut builder = self.wallet.build_tx();

        builder
            .set_recipients(recipients.into_iter().map(Into::into).collect())
            .fee_rate(fee_rate.into());

        let psbt = builder.finish()?;
        Ok(psbt.into())
    }

    pub fn sign(&self, psbt: Psbt) -> JsResult<bool> {
        self.wallet.sign(&mut psbt.into(), SignOptions::default())?;
        Ok(true)
    }
}
