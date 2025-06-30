use serde::Serialize;
use serde_wasm_bindgen::to_value;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen]
pub struct BdkError {
    code: BdkErrorCode,
    message: String,
    data: JsValue,
}

impl BdkError {
    pub fn new<D>(code: BdkErrorCode, message: impl Into<String>, data: D) -> Self
    where
        D: Serialize,
    {
        BdkError {
            code,
            message: message.into(),
            data: to_value(&data).unwrap_or(JsValue::UNDEFINED),
        }
    }
}

#[wasm_bindgen]
impl BdkError {
    #[wasm_bindgen(getter)]
    pub fn code(&self) -> BdkErrorCode {
        self.code
    }

    #[wasm_bindgen(getter)]
    pub fn message(&self) -> String {
        self.message.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn data(&self) -> JsValue {
        self.data.clone()
    }
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum BdkErrorCode {
    /// ------- Transaction creation errors -------

    /// There was a problem with the descriptors passed in
    Descriptor,
    /// There was a problem while extracting and manipulating policies
    Policy,
    /// Spending policy is not compatible with this [`KeychainKind`]
    SpendingPolicyRequired,
    /// Requested invalid transaction version '0'
    Version0,
    /// Requested transaction version `1`, but at least `2` is needed to use OP_CSV
    Version1Csv,
    /// Requested `LockTime` is less than is required to spend from this script
    LockTime,
    /// Cannot enable RBF with `Sequence` given a required OP_CSV
    RbfSequenceCsv,
    /// When bumping a tx the absolute fee requested is lower than replaced tx absolute fee
    FeeTooLow,
    /// When bumping a tx the fee rate requested is lower than required
    FeeRateTooLow,
    /// `manually_selected_only` option is selected but no utxo has been passed
    NoUtxosSelected,
    /// Output created is under the dust limit, 546 satoshis
    OutputBelowDustLimit,
    /// Wallet's UTXO set is not enough to cover recipient's requested plus fee.
    InsufficientFunds,
    /// Cannot build a tx without recipients
    NoRecipients,
    /// Partially signed bitcoin transaction error
    Psbt,
    /// In order to use the [`TxBuilder::add_global_xpubs`] option every extended
    /// key in the descriptor must either be a master key itself (having depth = 0) or have an
    /// explicit origin provided
    ///
    /// [`TxBuilder::add_global_xpubs`]: crate::wallet::tx_builder::TxBuilder::add_global_xpubs
    MissingKeyOrigin,
    /// Happens when trying to spend an UTXO that is not in the internal database
    UnknownUtxo,
    /// Missing non_witness_utxo on foreign utxo for given `OutPoint`
    MissingNonWitnessUtxo,
    /// Miniscript PSBT error
    MiniscriptPsbt,

    /// ------- Address errors -------

    /// Base58 error.
    Base58,
    /// Bech32 segwit decoding error.
    Bech32,
    /// A witness version conversion/parsing error.
    WitnessVersion,
    /// A witness program error.
    WitnessProgram,
    /// Tried to parse an unknown HRP.
    UnknownHrp,
    /// Legacy address is too long.
    LegacyAddressTooLong,
    /// Invalid base58 payload data length for legacy address.
    InvalidBase58PayloadLength,
    /// Invalid legacy address prefix in base58 data payload.
    InvalidLegacyPrefix,
    /// Address's network differs from required one.
    NetworkValidation,

    /// ------- Amount errors -------

    /// The amount is too big or too small.
    OutOfRange,
    /// Amount has higher precision than supported by the type.
    TooPrecise,
    /// A digit was expected but not found.
    MissingDigits,
    /// Input string was too large.
    InputTooLarge,
    /// Invalid character in input.
    InvalidCharacter,

    /// ------- Other errors -------
    /// Unexpected error, should never happen
    Unexpected,
}
