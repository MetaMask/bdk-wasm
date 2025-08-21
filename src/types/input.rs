use std::ops::Deref;
use wasm_bindgen::prelude::wasm_bindgen;

use bdk_wallet::bitcoin::TxIn as BdkTxIn;

use crate::types::{OutPoint, ScriptBuf};

/// Bitcoin transaction input.
///
/// It contains the location of the previous transaction's output,
/// that it spends and set of scripts that satisfy its spending
/// conditions.
#[wasm_bindgen]
#[derive(Clone)]

pub struct TxIn(BdkTxIn);

impl Deref for TxIn {
    type Target = BdkTxIn;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[wasm_bindgen]
impl TxIn {
    /// The reference to the previous output that is being used as an input.
    #[wasm_bindgen(getter)]
    pub fn previous_output(&self) -> OutPoint {
        self.0.previous_output.into()
    }

    /// The script which pushes values on the stack which will cause
    /// the referenced output's script to be accepted.
    #[wasm_bindgen(getter)]
    pub fn script_sig(&self) -> ScriptBuf {
        self.0.script_sig.clone().into()
    }

    /// Returns the base size of this input.
    ///
    /// Base size excludes the witness data.
    #[wasm_bindgen(getter)]
    pub fn base_size(&self) -> usize {
        self.0.base_size()
    }

    /// Returns the total number of bytes that this input contributes to a transaction.
    ///
    /// Total size includes the witness data.
    #[wasm_bindgen(getter)]
    pub fn total_size(&self) -> usize {
        self.0.total_size()
    }

    /// Returns true if this input enables the [`absolute::LockTime`] (aka `nLockTime`) of its
    /// [`Transaction`].
    ///
    /// `nLockTime` is enabled if *any* input enables it. See [`Transaction::is_lock_time_enabled`]
    ///  to check the overall state. If none of the inputs enables it, the lock time value is simply
    ///  ignored. If this returns false and OP_CHECKLOCKTIMEVERIFY is used in the redeem script with
    ///  this input then the script execution will fail [BIP-0065].
    #[wasm_bindgen(getter)]
    pub fn enables_lock_time(&self) -> bool {
        self.0.enables_lock_time()
    }
}

impl From<BdkTxIn> for TxIn {
    fn from(inner: BdkTxIn) -> Self {
        TxIn(inner)
    }
}

impl From<&BdkTxIn> for TxIn {
    fn from(inner: &BdkTxIn) -> Self {
        TxIn(inner.clone())
    }
}

impl From<TxIn> for BdkTxIn {
    fn from(txin: TxIn) -> Self {
        txin.0
    }
}
