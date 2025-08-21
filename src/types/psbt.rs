use bdk_wallet::serde_json::to_string;
use std::ops::{Deref, DerefMut};
use std::str::FromStr;

use bdk_wallet::{
    bitcoin::{Amount as BdkAmount, Psbt as BdkPsbt, ScriptBuf as BdkScriptBuf},
    psbt::PsbtUtils,
};

use wasm_bindgen::prelude::wasm_bindgen;

use crate::result::JsResult;
use crate::types::ScriptBuf;

use super::{Address, Amount, FeeRate, Transaction};

/// A Partially Signed Transaction.
#[wasm_bindgen]
#[derive(Clone)]
pub struct Psbt(BdkPsbt);

impl Deref for Psbt {
    type Target = BdkPsbt;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Psbt {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[wasm_bindgen]
impl Psbt {
    /// Extracts the [`Transaction`] from a [`Psbt`] by filling in the available signature information.
    ///
    /// ## Errors
    ///
    /// [`ExtractTxError`] variants will contain either the [`Psbt`] itself or the [`Transaction`]
    /// that was extracted. These can be extracted from the Errors in order to recover.
    /// See the error documentation for info on the variants. In general, it covers large fees.
    pub fn extract_tx_fee_rate_limit(self) -> JsResult<Transaction> {
        let tx = self.0.extract_tx_fee_rate_limit()?;
        Ok(tx.into())
    }

    /// An alias for [`extract_tx_fee_rate_limit`].
    pub fn extract_tx(self) -> JsResult<Transaction> {
        let tx = self.0.extract_tx()?;
        Ok(tx.into())
    }

    /// Extracts the [`Transaction`] from a [`Psbt`] by filling in the available signature information.
    pub fn extract_tx_with_fee_rate_limit(self, max_fee_rate: FeeRate) -> JsResult<Transaction> {
        let tx = self.0.extract_tx_with_fee_rate_limit(max_fee_rate.into())?;
        Ok(tx.into())
    }

    pub fn fee(&self) -> JsResult<Amount> {
        let fee = self.0.fee()?;
        Ok(fee.into())
    }

    /// The total transaction fee amount, sum of input amounts minus sum of output amounts, in sats.
    /// If the PSBT is missing a TxOut for an input returns None.
    pub fn fee_amount(&self) -> Option<Amount> {
        let fee_amount = self.0.fee_amount();
        fee_amount.map(Into::into)
    }

    /// The transaction's fee rate. This value will only be accurate if calculated AFTER the
    /// `Psbt` is finalized and all witness/signature data is added to the transaction.
    /// If the PSBT is missing a TxOut for an input returns None.
    pub fn fee_rate(&self) -> Option<FeeRate> {
        let fee_rate = self.0.fee_rate();
        fee_rate.map(Into::into)
    }

    /// The version number of this PSBT. If omitted, the version number is 0.
    #[wasm_bindgen(getter)]
    pub fn version(&self) -> u32 {
        self.0.version
    }

    /// Combines this [`Psbt`] with `other` PSBT as described by BIP 174. In-place.
    ///
    /// In accordance with BIP 174 this function is commutative i.e., `A.combine(B) == B.combine(A)`
    pub fn combine(&mut self, other: Psbt) -> JsResult<()> {
        self.0.combine(other.into())?;
        Ok(())
    }

    /// The unsigned transaction, scriptSigs and witnesses for each input must be empty.
    #[wasm_bindgen(getter)]
    pub fn unsigned_tx(&self) -> Transaction {
        self.0.unsigned_tx.clone().into()
    }

    /// Serialize the PSBT to a string in base64 format
    #[allow(clippy::inherent_to_string)]
    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&self) -> String {
        self.0.to_string()
    }

    /// Create a PSBT from a base64 string
    pub fn from_string(val: &str) -> JsResult<Psbt> {
        Ok(Psbt(BdkPsbt::from_str(val)?))
    }

    /// Serialize `Psbt` to JSON.
    pub fn to_json(&self) -> String {
        to_string(&self.0).expect("Serialization should not fail")
    }

    #[wasm_bindgen(js_name = clone)]
    pub fn js_clone(&self) -> Psbt {
        self.clone()
    }
}

impl From<BdkPsbt> for Psbt {
    fn from(inner: BdkPsbt) -> Self {
        Psbt(inner)
    }
}

impl From<Psbt> for BdkPsbt {
    fn from(psbt: Psbt) -> Self {
        psbt.0
    }
}

/// A Transaction recipient
#[wasm_bindgen]
#[derive(Clone)]
pub struct Recipient {
    script_pubkey: BdkScriptBuf,
    amount: BdkAmount,
}

#[wasm_bindgen]
impl Recipient {
    #[wasm_bindgen(constructor)]
    pub fn new(script_pubkey: ScriptBuf, amount: Amount) -> Self {
        Recipient {
            script_pubkey: script_pubkey.into(),
            amount: amount.into(),
        }
    }

    pub fn from_address(address: Address, amount: Amount) -> Self {
        Recipient {
            script_pubkey: address.script_pubkey().into(),
            amount: amount.into(),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn script_pubkey(&self) -> ScriptBuf {
        self.script_pubkey.clone().into()
    }

    #[wasm_bindgen(getter)]
    pub fn amount(&self) -> Amount {
        self.amount.into()
    }
}

impl From<Recipient> for (BdkScriptBuf, BdkAmount) {
    fn from(r: Recipient) -> Self {
        (r.script_pubkey.clone(), r.amount)
    }
}
