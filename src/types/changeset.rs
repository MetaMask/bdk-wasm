use std::ops::Deref;

use bdk_wallet::{
    chain::Merge,
    serde_json::{from_str, to_string},
    ChangeSet as BdkChangeSet,
};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::result::JsResult;

/// A changeset for [`Wallet`].
#[wasm_bindgen]
#[derive(PartialEq)]
pub struct ChangeSet(BdkChangeSet);

#[wasm_bindgen]
impl ChangeSet {
    /// Merge another [`ChangeSet`] into itself.
    pub fn merge(&mut self, other: ChangeSet) {
        self.0.merge(other.into());
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Serialize `ChangeSet` to JSON.
    pub fn to_json(&self) -> String {
        to_string(&self.0).expect("Serialization should not fail")
    }

    /// Create a new `ChangeSet` from a JSON string.
    pub fn from_json(val: &str) -> JsResult<ChangeSet> {
        Ok(ChangeSet(from_str(val)?))
    }
}

impl Deref for ChangeSet {
    type Target = BdkChangeSet;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<BdkChangeSet> for ChangeSet {
    fn from(inner: BdkChangeSet) -> Self {
        ChangeSet(inner)
    }
}

impl From<ChangeSet> for BdkChangeSet {
    fn from(changeset: ChangeSet) -> Self {
        changeset.0
    }
}
