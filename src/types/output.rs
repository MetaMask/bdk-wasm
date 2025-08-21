use bdk_wallet::{
    bitcoin::{OutPoint as BdkOutPoint, TxOut as BdkTxOut},
    LocalOutput as BdkLocalOutput,
};
use std::{ops::Deref, str::FromStr};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    result::JsResult,
    types::{Amount, KeychainKind, ScriptBuf},
};

use super::Txid;

/// A reference to a transaction output.
#[wasm_bindgen]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct OutPoint(BdkOutPoint);

impl Deref for OutPoint {
    type Target = BdkOutPoint;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[wasm_bindgen]
impl OutPoint {
    #[wasm_bindgen(constructor)]
    pub fn new(txid: Txid, vout: u32) -> Self {
        BdkOutPoint::new(txid.into(), vout).into()
    }

    pub fn from_string(outpoint_str: &str) -> JsResult<Self> {
        let outpoint = BdkOutPoint::from_str(outpoint_str)?;
        Ok(outpoint.into())
    }

    /// The index of the referenced output in its transaction's vout.
    #[wasm_bindgen(getter)]
    pub fn vout(&self) -> u32 {
        self.0.vout
    }

    /// The referenced transaction's txid.
    #[wasm_bindgen(getter)]
    pub fn txid(&self) -> Txid {
        self.0.txid.into()
    }

    #[allow(clippy::inherent_to_string)]
    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl From<BdkOutPoint> for OutPoint {
    fn from(inner: BdkOutPoint) -> Self {
        OutPoint(inner)
    }
}

impl From<OutPoint> for BdkOutPoint {
    fn from(outpoint: OutPoint) -> Self {
        outpoint.0
    }
}

/// Bitcoin transaction output.
///
/// Defines new coins to be created as a result of the transaction,
/// along with spending conditions ("script", aka "output script"),
/// which an input spending it must satisfy.
///
/// An output that is not yet spent by an input is called Unspent Transaction Output ("UTXO").
#[wasm_bindgen]
#[derive(Clone)]
pub struct TxOut(BdkTxOut);

impl Deref for TxOut {
    type Target = BdkTxOut;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[wasm_bindgen]
impl TxOut {
    /// The value of the output, in satoshis.
    #[wasm_bindgen(getter)]
    pub fn value(&self) -> Amount {
        self.0.value.into()
    }

    /// The script which must be satisfied for the output to be spent.
    #[wasm_bindgen(getter)]
    pub fn script_pubkey(&self) -> ScriptBuf {
        self.0.script_pubkey.clone().into()
    }

    /// Returns the total number of bytes that this output contributes to a transaction.
    ///
    /// There is no difference between base size vs total size for outputs.
    #[wasm_bindgen(getter)]
    pub fn size(&self) -> usize {
        self.0.size()
    }
}

impl From<BdkTxOut> for TxOut {
    fn from(inner: BdkTxOut) -> Self {
        TxOut(inner)
    }
}

impl From<&BdkTxOut> for TxOut {
    fn from(inner: &BdkTxOut) -> Self {
        TxOut(inner.clone())
    }
}

impl From<TxOut> for BdkTxOut {
    fn from(txout: TxOut) -> Self {
        txout.0
    }
}

/// A reference to a transaction output.
#[wasm_bindgen]
#[derive(Clone)]
pub struct LocalOutput(BdkLocalOutput);

impl Deref for LocalOutput {
    type Target = BdkLocalOutput;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[wasm_bindgen]
impl LocalOutput {
    /// Transaction output
    #[wasm_bindgen(getter)]
    pub fn txout(&self) -> TxOut {
        self.0.txout.clone().into()
    }

    /// The derivation index for the script pubkey in the wallet
    #[wasm_bindgen(getter)]
    pub fn derivation_index(&self) -> u32 {
        self.0.derivation_index
    }

    /// Reference to a transaction output
    #[wasm_bindgen(getter)]
    pub fn outpoint(&self) -> OutPoint {
        self.0.outpoint.into()
    }

    /// Type of keychain
    #[wasm_bindgen(getter)]
    pub fn keychain(&self) -> KeychainKind {
        self.0.keychain.into()
    }
}

impl From<BdkLocalOutput> for LocalOutput {
    fn from(inner: BdkLocalOutput) -> Self {
        LocalOutput(inner)
    }
}

impl From<LocalOutput> for BdkLocalOutput {
    fn from(output: LocalOutput) -> Self {
        output.0
    }
}
