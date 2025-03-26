use bdk_wallet::serde_json::to_string;
use std::ops::{Deref, DerefMut};
use std::str::FromStr;

use bdk_wallet::{
    bitcoin::{Amount as BdkAmount, Psbt as BdkPsbt, ScriptBuf as BdkScriptBuf},
    psbt::PsbtUtils,
};

use wasm_bindgen::prelude::wasm_bindgen;

use crate::result::JsResult;

use super::{Address, Amount, FeeRate, Transaction};

/// A Partially Signed Transaction.
#[wasm_bindgen]
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
    pub fn extract_tx(self) -> JsResult<Transaction> {
        let tx = self.0.extract_tx()?;
        Ok(tx.into())
    }

    pub fn extract_tx_with_fee_rate_limit(self, max_fee_rate: FeeRate) -> JsResult<Transaction> {
        let tx = self.0.extract_tx_with_fee_rate_limit(max_fee_rate.into())?;
        Ok(tx.into())
    }

    pub fn fee(&self) -> JsResult<Amount> {
        let fee = self.0.fee()?;
        Ok(fee.into())
    }

    pub fn fee_amount(&self) -> Option<Amount> {
        let fee_amount = self.0.fee_amount();
        fee_amount.map(Into::into)
    }

    pub fn fee_rate(&self) -> Option<FeeRate> {
        let fee_rate = self.0.fee_rate();
        fee_rate.map(Into::into)
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
    address: Address,
    amount: Amount,
}

#[wasm_bindgen]
impl Recipient {
    #[wasm_bindgen(constructor)]
    pub fn new(address: Address, amount: Amount) -> Self {
        Recipient { address, amount }
    }

    #[wasm_bindgen(getter)]
    pub fn address(&self) -> Address {
        self.address.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn amount(&self) -> Amount {
        self.amount
    }
}

impl From<Recipient> for (BdkScriptBuf, BdkAmount) {
    fn from(r: Recipient) -> Self {
        (r.address().script_pubkey(), r.amount().into())
    }
}
