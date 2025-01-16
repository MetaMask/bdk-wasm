use std::ops::Deref;

use bdk_core::spk_client::{
    FullScanRequest as BdkFullScanRequest, FullScanResponse as BdkFullScanResponse, SyncRequest as BdkSyncRequest,
    SyncResponse as BdkSyncResponse,
};
use bdk_wallet::{KeychainKind, Update as BdkUpdate};
use wasm_bindgen::prelude::wasm_bindgen;

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
#[derive(Debug)]
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
