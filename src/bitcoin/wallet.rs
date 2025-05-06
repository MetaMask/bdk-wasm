use std::{cell::RefCell, rc::Rc};

use bdk_wallet::{SignOptions as BdkSignOptions, Wallet as BdkWallet};
use js_sys::Date;
use wasm_bindgen::{prelude::wasm_bindgen, JsError};

use crate::{
    bitcoin::WalletTx,
    result::JsResult,
    types::{
        AddressInfo, Amount, Balance, ChangeSet, CheckPoint, FeeRate, FullScanRequest, KeychainKind, LocalOutput,
        Network, OutPoint, Psbt, ScriptBuf, SentAndReceived, SpkIndexed, SyncRequest, Transaction, Txid, Update,
    },
};

use super::{TxBuilder, UnconfirmedTx};

// We wrap a `BdkWallet` in `Rc<RefCell<...>>` because `wasm_bindgen` do not
// support Rust's lifetimes. This allows us to forward a reference to the
// internal wallet when using `build_tx` and to enforce the lifetime at runtime
// and to preserve "safe mutability".
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

    #[wasm_bindgen(getter)]
    pub fn network(&self) -> Network {
        self.0.borrow().network().into()
    }

    #[wasm_bindgen(getter)]
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

    pub fn list_unspent(&self) -> Vec<LocalOutput> {
        self.0.borrow().list_unspent().map(Into::into).collect()
    }

    pub fn list_output(&self) -> Vec<LocalOutput> {
        self.0.borrow().list_output().map(Into::into).collect()
    }

    pub fn get_utxo(&self, op: OutPoint) -> Option<LocalOutput> {
        self.0.borrow().get_utxo(op.into()).map(Into::into)
    }

    pub fn transactions(&self) -> Vec<WalletTx> {
        self.0.borrow().transactions().map(Into::into).collect()
    }

    pub fn get_tx(&self, txid: Txid) -> Option<WalletTx> {
        self.0.borrow().get_tx(txid.into()).map(Into::into)
    }

    #[wasm_bindgen(getter)]
    pub fn latest_checkpoint(&self) -> CheckPoint {
        self.0.borrow().latest_checkpoint().into()
    }

    pub fn take_staged(&self) -> Option<ChangeSet> {
        self.0.borrow_mut().take_staged().map(Into::into)
    }

    pub fn public_descriptor(&self, keychain: KeychainKind) -> String {
        self.0.borrow().public_descriptor(keychain.into()).to_string()
    }

    pub fn sign(&self, psbt: &mut Psbt, options: SignOptions) -> JsResult<bool> {
        let result = self.0.borrow().sign(psbt, options.into())?;
        Ok(result)
    }

    pub fn derivation_index(&self, keychain: KeychainKind) -> Option<u32> {
        self.0.borrow().derivation_index(keychain.into())
    }

    pub fn build_tx(&self) -> TxBuilder {
        TxBuilder::new(self.0.clone())
    }

    pub fn calculate_fee(&self, tx: Transaction) -> JsResult<Amount> {
        let fee = self.0.borrow().calculate_fee(&tx.into())?;
        Ok(fee.into())
    }

    pub fn calculate_fee_rate(&self, tx: Transaction) -> JsResult<FeeRate> {
        let fee_rate = self.0.borrow().calculate_fee_rate(&tx.into())?;
        Ok(fee_rate.into())
    }

    pub fn sent_and_received(&self, tx: Transaction) -> JsResult<SentAndReceived> {
        let (sent, received) = self.0.borrow().sent_and_received(&tx.into());
        Ok(SentAndReceived(sent.into(), received.into()))
    }

    pub fn is_mine(&self, script: ScriptBuf) -> bool {
        self.0.borrow().is_mine(script.into())
    }

    pub fn derivation_of_spk(&self, spk: ScriptBuf) -> Option<SpkIndexed> {
        self.0
            .borrow()
            .derivation_of_spk(spk.into())
            .map(|(keychain, index)| SpkIndexed(keychain.into(), index))
    }

    pub fn apply_unconfirmed_txs(&self, unconfirmed_txs: Vec<UnconfirmedTx>) {
        self.0
            .borrow_mut()
            .apply_unconfirmed_txs(unconfirmed_txs.into_iter().map(Into::into))
    }
}

#[wasm_bindgen]
pub struct SignOptions(BdkSignOptions);

#[wasm_bindgen]
impl SignOptions {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        SignOptions(BdkSignOptions::default())
    }

    #[wasm_bindgen(getter)]
    pub fn trust_witness_utxo(&self) -> bool {
        self.0.trust_witness_utxo
    }

    #[wasm_bindgen(setter)]
    pub fn set_trust_witness_utxo(&mut self, value: bool) {
        self.0.trust_witness_utxo = value;
    }

    #[wasm_bindgen(getter)]
    pub fn assume_height(&self) -> Option<u32> {
        self.0.assume_height
    }

    #[wasm_bindgen(setter)]
    pub fn set_assume_height(&mut self, value: Option<u32>) {
        self.0.assume_height = value;
    }

    #[wasm_bindgen(getter)]
    pub fn allow_all_sighashes(&self) -> bool {
        self.0.allow_all_sighashes
    }

    #[wasm_bindgen(setter)]
    pub fn set_allow_all_sighashes(&mut self, value: bool) {
        self.0.allow_all_sighashes = value;
    }

    #[wasm_bindgen(getter)]
    pub fn try_finalize(&self) -> bool {
        self.0.try_finalize
    }

    #[wasm_bindgen(setter)]
    pub fn set_try_finalize(&mut self, value: bool) {
        self.0.try_finalize = value;
    }

    #[wasm_bindgen(getter)]
    pub fn sign_with_tap_internal_key(&self) -> bool {
        self.0.sign_with_tap_internal_key
    }

    #[wasm_bindgen(setter)]
    pub fn set_sign_with_tap_internal_key(&mut self, value: bool) {
        self.0.sign_with_tap_internal_key = value;
    }

    #[wasm_bindgen(getter)]
    pub fn allow_grinding(&self) -> bool {
        self.0.allow_grinding
    }

    #[wasm_bindgen(setter)]
    pub fn set_allow_grinding(&mut self, value: bool) {
        self.0.allow_grinding = value;
    }
}

impl From<SignOptions> for BdkSignOptions {
    fn from(options: SignOptions) -> Self {
        options.0
    }
}

impl Default for SignOptions {
    fn default() -> Self {
        Self::new()
    }
}
