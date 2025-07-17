use std::{collections::BTreeSet, sync::Arc};

use bdk_wallet::{
    bitcoin::{Transaction as BdkTransaction, Txid as BdkTxid},
    chain::{ChainPosition as BdkChainPosition, ConfirmationBlockTime as BdkConfirmationBlockTime},
    WalletTx as BdkWalletTx,
};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::types::{ChainPosition, ConfirmationBlockTime, Transaction, Txid};

/// A Transaction managed by a `Wallet`.
#[wasm_bindgen]
pub struct WalletTx {
    txid: BdkTxid,
    tx: BdkTransaction,
    anchors: BTreeSet<BdkConfirmationBlockTime>,
    last_seen: Option<u64>,
    first_seen: Option<u64>,
    chain_position: BdkChainPosition<BdkConfirmationBlockTime>,
}

#[wasm_bindgen]
impl WalletTx {
    /// Txid of the transaction.
    #[wasm_bindgen(getter)]
    pub fn txid(&self) -> Txid {
        self.txid.into()
    }

    /// A partial or full representation of the transaction.
    #[wasm_bindgen(getter)]
    pub fn tx(&self) -> Transaction {
        self.tx.clone().into()
    }

    /// The blocks that the transaction is "anchored" in.
    #[wasm_bindgen(getter)]
    pub fn anchors(&self) -> Vec<ConfirmationBlockTime> {
        self.anchors.iter().map(Into::into).collect()
    }

    /// The last-seen unix timestamp of the transaction as unconfirmed.
    #[wasm_bindgen(getter)]
    pub fn last_seen(&self) -> Option<u64> {
        self.last_seen
    }

    /// The first-seen unix timestamp of the transaction as unconfirmed.
    #[wasm_bindgen(getter)]
    pub fn first_seen(&self) -> Option<u64> {
        self.first_seen
    }

    /// How the transaction is observed in the canonical chain (confirmed or unconfirmed).
    #[wasm_bindgen(getter)]
    pub fn chain_position(&self) -> ChainPosition {
        self.chain_position.into()
    }
}

impl From<BdkWalletTx<'_>> for WalletTx {
    fn from(tx: BdkWalletTx) -> Self {
        WalletTx {
            txid: tx.tx_node.txid,
            tx: tx.tx_node.tx.as_ref().clone(),
            anchors: tx.tx_node.anchors.clone(),
            last_seen: tx.tx_node.last_seen,
            first_seen: tx.tx_node.first_seen,
            chain_position: tx.chain_position,
        }
    }
}

#[wasm_bindgen]
pub struct UnconfirmedTx(Transaction, pub u64);

#[wasm_bindgen]
impl UnconfirmedTx {
    /// `tx` – your wrapped `Transaction`
    /// `last_seen` – unix epoch seconds (same convention as the rest of the API)
    #[wasm_bindgen(constructor)]
    pub fn new(tx: Transaction, last_seen: u64) -> UnconfirmedTx {
        UnconfirmedTx(tx, last_seen)
    }

    #[wasm_bindgen(getter)]
    pub fn tx(&self) -> Transaction {
        self.0.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn last_seen(&self) -> u64 {
        self.1
    }
}

impl From<UnconfirmedTx> for (Arc<BdkTransaction>, u64) {
    fn from(uctx: UnconfirmedTx) -> Self {
        (Arc::new(uctx.0.into()), uctx.1)
    }
}
