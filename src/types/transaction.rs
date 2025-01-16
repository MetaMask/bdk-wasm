use std::ops::Deref;

use bitcoin::Transaction as BdkTransaction;
use wasm_bindgen::prelude::wasm_bindgen;

/// Bitcoin transaction.
///
/// An authenticated movement of coins.
#[wasm_bindgen]
#[derive(Debug)]
pub struct Transaction(BdkTransaction);

impl Deref for Transaction {
    type Target = BdkTransaction;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[wasm_bindgen]
impl Transaction {
    pub fn compute_txid(self) -> String {
        self.0.compute_txid().as_raw_hash().to_string()
    }
}

impl From<BdkTransaction> for Transaction {
    fn from(inner: BdkTransaction) -> Self {
        Transaction(inner)
    }
}

impl From<Transaction> for BdkTransaction {
    fn from(tx: Transaction) -> Self {
        tx.0
    }
}
