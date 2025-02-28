use std::collections::BTreeSet;

use bdk_core::ConfirmationBlockTime as BdkConfirmationBlockTime;
use bdk_wallet::chain::ChainPosition as BdkChainPosition;
use bdk_wallet::WalletTx as BdkWalletTx;
use bitcoin::{Transaction as BdkTransaction, Txid as BdkTxid};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::types::{ChainPosition, ConfirmationBlockTime, Transaction, Txid};

/// A Transaction managed by a `Wallet`.
#[wasm_bindgen]
pub struct WalletTx {
    txid: BdkTxid,
    tx: BdkTransaction,
    anchors: BTreeSet<BdkConfirmationBlockTime>,
    last_seen_unconfirmed: Option<u64>,
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
    pub fn last_seen_unconfirmed(&self) -> Option<u64> {
        self.last_seen_unconfirmed
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
            last_seen_unconfirmed: tx.tx_node.last_seen_unconfirmed,
            chain_position: tx.chain_position,
        }
    }
}
