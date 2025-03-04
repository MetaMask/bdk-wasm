use std::ops::Deref;

use bdk_wallet::chain::TxGraph as BdkTxGraph;
use wasm_bindgen::prelude::wasm_bindgen;

use super::{OutPoint, TxOut};

/// A graph of transactions and spends.
#[wasm_bindgen]
pub struct TxGraph(BdkTxGraph);

impl Deref for TxGraph {
    type Target = BdkTxGraph;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[wasm_bindgen]
impl TxGraph {
    pub fn get_txout(&self, outpoint: OutPoint) -> Option<TxOut> {
        self.0.get_txout(outpoint.into()).map(Into::into)
    }
}

impl From<BdkTxGraph> for TxGraph {
    fn from(inner: BdkTxGraph) -> Self {
        TxGraph(inner)
    }
}

impl From<&BdkTxGraph> for TxGraph {
    fn from(inner: &BdkTxGraph) -> Self {
        TxGraph(inner.clone())
    }
}

impl From<TxGraph> for BdkTxGraph {
    fn from(graph: TxGraph) -> Self {
        graph.0
    }
}
