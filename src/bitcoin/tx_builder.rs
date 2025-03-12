use std::{cell::RefCell, rc::Rc};

use bdk_wallet::Wallet as BdkWallet;
use bitcoin::ScriptBuf;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    result::JsResult,
    types::{Address, FeeRate, OutPoint, Psbt, Recipient},
};

/// A transaction builder.
///
/// A `TxBuilder` is created by calling [`build_tx`] or [`build_fee_bump`] on a wallet. After
/// assigning it, you set options on it until finally calling [`finish`] to consume the builder and
/// generate the transaction.
///
/// Each option setting method on `TxBuilder` takes and returns a new builder so you can chain calls
#[wasm_bindgen]
pub struct TxBuilder {
    wallet: Rc<RefCell<BdkWallet>>,
    recipients: Vec<Recipient>,
    unspendable: Vec<OutPoint>,
    fee_rate: FeeRate,
    drain_wallet: bool,
    drain_to: Option<ScriptBuf>,
    allow_dust: bool,
}

#[wasm_bindgen]
impl TxBuilder {
    // We make this constructor only visible to the crate to hide the use of the `Rc<RefCell<BdkWallet>>` in `Wallet::build_tx`.
    pub(crate) fn new(wallet: Rc<RefCell<BdkWallet>>) -> TxBuilder {
        TxBuilder {
            wallet,
            recipients: vec![],
            unspendable: vec![],
            fee_rate: FeeRate::new(1),
            drain_wallet: false,
            allow_dust: false,
            drain_to: None,
        }
    }

    /// Replace the recipients already added with a new list
    pub fn set_recipients(mut self, recipients: Vec<Recipient>) -> Self {
        self.recipients = recipients;
        self
    }

    /// Add a recipient to the internal list
    pub fn add_recipient(mut self, recipient: Recipient) -> Self {
        self.recipients.push(recipient);
        self
    }

    /// Replace the internal list of unspendable utxos with a new list
    pub fn unspendable(mut self, unspendable: Vec<OutPoint>) -> Self {
        self.unspendable = unspendable;
        self
    }

    /// Add a utxo to the internal list of unspendable utxos
    pub fn add_unspendable(mut self, outpoint: OutPoint) -> Self {
        self.unspendable.push(outpoint);
        self
    }

    /// Set a custom fee rate.
    ///
    /// This method sets the mining fee paid by the transaction as a rate on its size.
    /// This means that the total fee paid is equal to `fee_rate` times the size
    /// of the transaction. Default is 1 sat/vB in accordance with Bitcoin Core's default
    /// relay policy.
    ///
    /// Note that this is really a minimum feerate -- it's possible to
    /// overshoot it slightly since adding a change output to drain the remaining
    /// excess might not be viable.
    pub fn fee_rate(mut self, fee_rate: FeeRate) -> Self {
        self.fee_rate = fee_rate;
        self
    }

    /// Spend all the available inputs. This respects filters like [`TxBuilder::unspendable`] and the change policy.
    pub fn drain_wallet(mut self) -> Self {
        self.drain_wallet = true;
        self
    }

    /// Sets the address to *drain* excess coins to.
    ///
    /// Usually, when there are excess coins they are sent to a change address generated by the
    /// wallet. This option replaces the usual change address with an arbitrary `script_pubkey` of
    /// your choosing. Just as with a change output, if the drain output is not needed (the excess
    /// coins are too small) it will not be included in the resulting transaction. The only
    /// difference is that it is valid to use `drain_to` without setting any ordinary recipients
    /// with [`add_recipient`] (but it is perfectly fine to add recipients as well).
    ///
    /// If you choose not to set any recipients, you should provide the utxos that the
    /// transaction should spend via [`add_utxos`].
    pub fn drain_to(mut self, address: Address) -> Self {
        self.drain_to = Some(address.script_pubkey());
        self
    }

    /// Set whether or not the dust limit is checked.
    ///
    /// **Note**: by avoiding a dust limit check you may end up with a transaction that is non-standard.
    pub fn allow_dust(mut self, allow_dust: bool) -> Self {
        self.allow_dust = allow_dust;
        self
    }

    /// Finish building the transaction.
    ///
    /// Returns a new [`Psbt`] per [`BIP174`].
    pub fn finish(self) -> JsResult<Psbt> {
        let mut wallet = self.wallet.borrow_mut();
        let mut builder = wallet.build_tx();

        builder
            .set_recipients(self.recipients.into_iter().map(Into::into).collect())
            .unspendable(self.unspendable.into_iter().map(Into::into).collect())
            .fee_rate(self.fee_rate.into())
            .allow_dust(self.allow_dust);

        if self.drain_wallet {
            builder.drain_wallet();
        }

        if let Some(drain_recipient) = self.drain_to {
            builder.drain_to(drain_recipient);
        }

        let psbt = builder.finish()?;
        Ok(psbt.into())
    }
}
