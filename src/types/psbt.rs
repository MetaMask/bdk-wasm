use std::ops::{Deref, DerefMut};

use bdk_wallet::psbt::PsbtUtils;
use bitcoin::{Amount as BdkAmount, Psbt as BdkPsbt, ScriptBuf as BdkScriptBuf};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::result::JsResult;

use super::{Address, Amount, FeeRate, Transaction};

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

    #[wasm_bindgen]
    pub fn fee(&self) -> JsResult<Amount> {
        let fee = self.0.fee()?;
        Ok(fee.into())
    }

    #[wasm_bindgen]
    pub fn fee_amount(&self) -> Option<Amount> {
        let fee_amount = self.0.fee_amount();
        fee_amount.map(Into::into)
    }

    #[wasm_bindgen]
    pub fn fee_rate(&self) -> Option<FeeRate> {
        let fee_rate = self.0.fee_rate();
        fee_rate.map(Into::into)
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
        self.amount.clone()
    }
}

impl From<Recipient> for (BdkScriptBuf, BdkAmount) {
    fn from(r: Recipient) -> Self {
        (r.address().script_pubkey(), r.amount().into())
    }
}
