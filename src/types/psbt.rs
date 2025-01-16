use std::ops::Deref;

use bitcoin::{Amount as BdkAmount, FeeRate as BdkFeeRate, Psbt as BdkPsbt, ScriptBuf};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::result::JsResult;

use super::{AddressInfo, Amount, Transaction};

/// A Partially Signed Transaction.
#[wasm_bindgen]
#[derive(Debug)]
pub struct Psbt(BdkPsbt);

impl Deref for Psbt {
    type Target = BdkPsbt;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[wasm_bindgen]
impl Psbt {
    pub fn extract_tx(self) -> JsResult<Transaction> {
        let tx = self.0.extract_tx()?;
        Ok(tx.into())
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
#[derive(Debug)]
pub struct Recipient {
    address: AddressInfo,
    amount: Amount,
}

#[wasm_bindgen]
impl Recipient {
    #[wasm_bindgen(constructor)]
    pub fn new(address: AddressInfo, amount: Amount) -> Self {
        Recipient { address, amount }
    }

    #[wasm_bindgen(getter)]
    pub fn address(&self) -> AddressInfo {
        self.address.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn amount(&self) -> Amount {
        self.amount.clone()
    }
}

impl From<Recipient> for (ScriptBuf, BdkAmount) {
    fn from(r: Recipient) -> Self {
        (r.address().script_pubkey(), r.amount().into())
    }
}

/// Represents fee rate.
///
/// This is an integer newtype representing fee rate in `sat/kwu`. It provides protection against mixing
/// up the types as well as basic formatting features.
#[wasm_bindgen]
#[derive(Debug)]
pub struct FeeRate(BdkFeeRate);

impl Deref for FeeRate {
    type Target = BdkFeeRate;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<BdkFeeRate> for FeeRate {
    fn from(inner: BdkFeeRate) -> Self {
        FeeRate(inner)
    }
}

impl From<FeeRate> for BdkFeeRate {
    fn from(fee_rate: FeeRate) -> Self {
        fee_rate.0
    }
}
