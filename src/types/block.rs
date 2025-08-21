use std::ops::Deref;

use bdk_wallet::chain::{BlockId as BdkBlockId, ConfirmationBlockTime as BdkConfirmationBlockTime};
use wasm_bindgen::prelude::wasm_bindgen;

/// A reference to a block in the canonical chain.
#[wasm_bindgen]
#[derive(Clone)]
pub struct BlockId(BdkBlockId);

#[wasm_bindgen]
impl BlockId {
    /// The height of the block.
    #[wasm_bindgen(getter)]
    pub fn height(&self) -> u32 {
        self.0.height
    }

    /// The hash of the block.
    #[wasm_bindgen(getter)]
    pub fn hash(&self) -> String {
        self.0.hash.to_string()
    }
}

impl From<BdkBlockId> for BlockId {
    fn from(inner: BdkBlockId) -> Self {
        BlockId(inner)
    }
}

/// Represents the observed position of some chain data.
#[wasm_bindgen]
pub struct ConfirmationBlockTime(BdkConfirmationBlockTime);

impl Deref for ConfirmationBlockTime {
    type Target = BdkConfirmationBlockTime;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[wasm_bindgen]
impl ConfirmationBlockTime {
    /// The anchor block.
    #[wasm_bindgen(getter)]
    pub fn block_id(&self) -> BlockId {
        self.0.block_id.into()
    }

    /// The confirmation time of the transaction being anchored.
    #[wasm_bindgen(getter)]
    pub fn confirmation_time(&self) -> u64 {
        self.0.confirmation_time
    }
}

impl From<&BdkConfirmationBlockTime> for ConfirmationBlockTime {
    fn from(inner: &BdkConfirmationBlockTime) -> Self {
        ConfirmationBlockTime(*inner)
    }
}

impl From<ConfirmationBlockTime> for BdkConfirmationBlockTime {
    fn from(conf_block_time: ConfirmationBlockTime) -> Self {
        conf_block_time.0
    }
}
