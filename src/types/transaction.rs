use std::{ops::Deref, str::FromStr};
use wasm_bindgen::prelude::wasm_bindgen;

use bitcoin::{OutPoint as BdkOutpoint, Transaction as BdkTransaction, Txid as BdkTxid};

use crate::result::JsResult;

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
    pub fn compute_txid(&self) -> Txid {
        self.0.compute_txid().into()
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

/// A reference to a transaction output.
#[wasm_bindgen]
pub struct Outpoint(BdkOutpoint);

impl Deref for Outpoint {
    type Target = BdkOutpoint;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[wasm_bindgen]
impl Outpoint {
    #[wasm_bindgen(constructor)]
    pub fn new(txid: Txid, vout: u32) -> Self {
        BdkOutpoint::new(txid.into(), vout).into()
    }
}

impl From<BdkOutpoint> for Outpoint {
    fn from(inner: BdkOutpoint) -> Self {
        Outpoint(inner)
    }
}

impl From<Outpoint> for BdkOutpoint {
    fn from(outpoint: Outpoint) -> Self {
        outpoint.0
    }
}

/// A bitcoin transaction hash/transaction ID.
#[wasm_bindgen]
#[derive(Debug)]
pub struct Txid(BdkTxid);

impl Deref for Txid {
    type Target = BdkTxid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[wasm_bindgen]
impl Txid {
    pub fn new(hash: String) -> JsResult<Self> {
        Ok(Txid(BdkTxid::from_str(&hash)?))
    }

    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl From<BdkTxid> for Txid {
    fn from(inner: BdkTxid) -> Self {
        Txid(inner)
    }
}

impl From<Txid> for BdkTxid {
    fn from(txid: Txid) -> Self {
        txid.0
    }
}
