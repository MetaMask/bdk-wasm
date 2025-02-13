use std::{cell::RefCell, rc::Rc};

use bdk_wallet::{SignOptions, Wallet as BdkWallet};
use js_sys::Date;
use wasm_bindgen::{prelude::wasm_bindgen, JsError};

use crate::{
    result::JsResult,
    types::{
        AddressInfo, Balance, ChangeSet, CheckPoint, FullScanRequest, KeychainKind, Network, Psbt, SyncRequest, Update,
    },
};

use super::TxBuilder;

#[wasm_bindgen]
pub struct Wallet(Rc<RefCell<BdkWallet>>);

#[wasm_bindgen]
impl Wallet {
    pub fn create(network: Network, external_descriptor: String, internal_descriptor: String) -> JsResult<Wallet> {
        let wallet = BdkWallet::create(external_descriptor, internal_descriptor)
            .network(network.into())
            .create_wallet_no_persist()?;

        Ok(Wallet(Rc::new(RefCell::new(wallet))))
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

        Ok(Wallet(Rc::new(RefCell::new(wallet))))
    }

    pub fn start_full_scan(&self) -> FullScanRequest {
        self.0.borrow().start_full_scan().build().into()
    }

    pub fn start_sync_with_revealed_spks(&self) -> SyncRequest {
        self.0.borrow().start_sync_with_revealed_spks().build().into()
    }

    pub fn apply_update(&self, update: Update) -> JsResult<()> {
        self.apply_update_at(update, (Date::now() / 1000.0) as u64)
    }

    pub fn apply_update_at(&self, update: Update, seen_at: u64) -> JsResult<()> {
        self.0.borrow_mut().apply_update_at(update, seen_at)?;
        Ok(())
    }

    pub fn network(&self) -> Network {
        self.0.borrow().network().into()
    }

    pub fn balance(&self) -> Balance {
        self.0.borrow().balance().into()
    }

    pub fn next_unused_address(&self, keychain: KeychainKind) -> AddressInfo {
        self.0.borrow_mut().next_unused_address(keychain.into()).into()
    }

    pub fn peek_address(&self, keychain: KeychainKind, index: u32) -> AddressInfo {
        self.0.borrow().peek_address(keychain.into(), index).into()
    }

    pub fn reveal_next_address(&self, keychain: KeychainKind) -> AddressInfo {
        self.0.borrow_mut().reveal_next_address(keychain.into()).into()
    }

    pub fn reveal_addresses_to(&self, keychain: KeychainKind, index: u32) -> Vec<AddressInfo> {
        self.0
            .borrow_mut()
            .reveal_addresses_to(keychain.into(), index)
            .map(Into::into)
            .collect()
    }

    pub fn list_unused_addresses(&self, keychain: KeychainKind) -> Vec<AddressInfo> {
        self.0
            .borrow()
            .list_unused_addresses(keychain.into())
            .map(Into::into)
            .collect()
    }

    pub fn latest_checkpoint(&self) -> CheckPoint {
        self.0.borrow().latest_checkpoint().into()
    }

    pub fn take_staged(&self) -> Option<ChangeSet> {
        self.0.borrow_mut().take_staged().map(Into::into)
    }

    pub fn public_descriptor(&self, keychain: KeychainKind) -> String {
        self.0.borrow().public_descriptor(keychain.into()).to_string()
    }

    pub fn sign(&self, psbt: &mut Psbt) -> JsResult<bool> {
        let result = self.0.borrow().sign(psbt, SignOptions::default())?;
        Ok(result)
    }

    pub fn derivation_index(&self, keychain: KeychainKind) -> Option<u32> {
        self.0.borrow().derivation_index(keychain.into())
    }

    pub fn build_tx(&self) -> TxBuilder {
        TxBuilder::new(self.0.clone())
    }
}
