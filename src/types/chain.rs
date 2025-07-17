use std::ops::Deref;

use bdk_wallet::{
    chain::{
        spk_client::{
            FullScanRequest as BdkFullScanRequest, FullScanResponse as BdkFullScanResponse,
            SyncRequest as BdkSyncRequest, SyncResponse as BdkSyncResponse,
        },
        ChainPosition as BdkChainPosition, ConfirmationBlockTime as BdkConfirmationBlockTime,
    },
    KeychainKind, Update as BdkUpdate,
};
use wasm_bindgen::prelude::wasm_bindgen;

use super::{ConfirmationBlockTime, Txid};

/// Data required to perform a spk-based blockchain client sync.
///
/// A client sync fetches relevant chain data for a known list of scripts, transaction ids and
/// outpoints.
#[wasm_bindgen]
pub struct SyncRequest(BdkSyncRequest<(KeychainKind, u32)>);

impl Deref for SyncRequest {
    type Target = BdkSyncRequest<(KeychainKind, u32)>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<BdkSyncRequest<(KeychainKind, u32)>> for SyncRequest {
    fn from(inner: BdkSyncRequest<(KeychainKind, u32)>) -> Self {
        SyncRequest(inner)
    }
}

impl From<SyncRequest> for BdkSyncRequest<(KeychainKind, u32)> {
    fn from(request: SyncRequest) -> Self {
        request.0
    }
}

/// Data required to perform a spk-based blockchain client full scan.
///
/// A client full scan iterates through all the scripts for the given keychains, fetching relevant
/// data until some stop gap number of scripts is found that have no data. This operation is
/// generally only used when importing or restoring previously used keychains in which the list of
/// used scripts is not known.
#[wasm_bindgen]
pub struct FullScanRequest(BdkFullScanRequest<KeychainKind>);

impl Deref for FullScanRequest {
    type Target = BdkFullScanRequest<KeychainKind>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<BdkFullScanRequest<KeychainKind>> for FullScanRequest {
    fn from(inner: BdkFullScanRequest<KeychainKind>) -> Self {
        FullScanRequest(inner)
    }
}

impl From<FullScanRequest> for BdkFullScanRequest<KeychainKind> {
    fn from(request: FullScanRequest) -> Self {
        request.0
    }
}

/// An update to [`Wallet`].
#[wasm_bindgen]
pub struct Update(BdkUpdate);

impl Deref for Update {
    type Target = BdkUpdate;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<BdkUpdate> for Update {
    fn from(inner: BdkUpdate) -> Self {
        Update(inner)
    }
}

impl From<Update> for BdkUpdate {
    fn from(update: Update) -> Self {
        update.0
    }
}

impl From<BdkFullScanResponse<KeychainKind>> for Update {
    fn from(result: BdkFullScanResponse<KeychainKind>) -> Self {
        Update(result.into())
    }
}

impl From<BdkSyncResponse> for Update {
    fn from(result: BdkSyncResponse) -> Self {
        Update(result.into())
    }
}

/// Represents the observed position of some chain data.
#[wasm_bindgen]
pub struct ChainPosition(BdkChainPosition<BdkConfirmationBlockTime>);

impl Deref for ChainPosition {
    type Target = BdkChainPosition<BdkConfirmationBlockTime>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[wasm_bindgen]
impl ChainPosition {
    /// Returns whether [`ChainPosition`] is confirmed or not.
    #[wasm_bindgen(getter)]
    pub fn is_confirmed(&self) -> bool {
        self.0.is_confirmed()
    }

    /// Determines the upper bound of the confirmation height.
    #[wasm_bindgen(getter)]
    pub fn confirmation_height_upper_bound(&self) -> Option<u32> {
        self.0.confirmation_height_upper_bound()
    }

    /// When the chain data is last seen in the mempool.
    ///
    /// This value will be `None` if the chain data was never seen in the mempool and only seen
    /// in a conflicting chain.
    #[wasm_bindgen(getter)]
    pub fn last_seen(&self) -> Option<u64> {
        match &self.0 {
            BdkChainPosition::Unconfirmed {
                first_seen: _,
                last_seen,
            } => *last_seen,
            _ => None,
        }
    }

    /// When the chain data was first seen in the mempool.
    ///
    /// This value will be `None` if the chain data was never seen in the mempool.
    #[wasm_bindgen(getter)]
    pub fn first_seen(&self) -> Option<u64> {
        match &self.0 {
            BdkChainPosition::Unconfirmed {
                first_seen,
                last_seen: _,
            } => *first_seen,
            _ => None,
        }
    }

    /// The [`Anchor`].
    #[wasm_bindgen(getter)]
    pub fn anchor(&self) -> Option<ConfirmationBlockTime> {
        match &self.0 {
            BdkChainPosition::Confirmed {
                anchor,
                transitively: _,
            } => Some(anchor.into()),
            _ => None,
        }
    }

    /// Whether the chain data is anchored transitively by a child transaction.
    ///
    /// If the value is `Some`, it means we have incomplete data. We can only deduce that the
    /// chain data is confirmed at a block equal to or lower than the block referenced by `A`.
    #[wasm_bindgen(getter)]
    pub fn transitively(&self) -> Option<Txid> {
        match &self.0 {
            BdkChainPosition::Confirmed {
                anchor: _,
                transitively,
            } => transitively.map(Into::into),
            _ => None,
        }
    }
}

impl From<BdkChainPosition<BdkConfirmationBlockTime>> for ChainPosition {
    fn from(inner: BdkChainPosition<BdkConfirmationBlockTime>) -> Self {
        ChainPosition(inner)
    }
}

impl From<ChainPosition> for BdkChainPosition<BdkConfirmationBlockTime> {
    fn from(chain_position: ChainPosition) -> Self {
        chain_position.0
    }
}
