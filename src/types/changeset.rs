use std::ops::Deref;

use bdk_core::Merge;
use bdk_wallet::{
    serde_json::{from_str, to_string},
    ChangeSet as BdkChangeSet,
};
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use crate::result::JsResult;

/// A changeset for [`Wallet`].
#[wasm_bindgen]
#[derive(Debug, PartialEq)]
pub struct ChangeSet {
    changeset: BdkChangeSet,
}

#[wasm_bindgen]
impl ChangeSet {
    /// Merge another [`ChangeSet`] into itself.
    pub fn merge(&mut self, other: ChangeSet) {
        self.changeset.merge(other.into());
    }

    pub fn is_empty(&self) -> bool {
        self.changeset.is_empty()
    }

    /// Serialize `ChangeSet` to JSON.
    pub fn to_json(&self) -> String {
        to_string(&self.changeset).expect("Serialization should not fail")
    }

    /// Serialize `ChangeSet` to JSON compatible with WASM.
    pub fn to_js(&self) -> JsResult<JsValue> {
        to_value(&self.changeset).map_err(Into::into)
    }

    /// Create a new `ChangeSet` from a JSON string.
    #[wasm_bindgen]
    pub fn from_json(val: &str) -> JsResult<ChangeSet> {
        Ok(ChangeSet {
            changeset: from_str(val)?,
        })
    }

    /// Create a new `ChangeSet` from a JS object.
    #[wasm_bindgen]
    pub fn from_js(js_value: JsValue) -> JsResult<ChangeSet> {
        Ok(ChangeSet {
            changeset: from_value(js_value)?,
        })
    }
}

impl Deref for ChangeSet {
    type Target = BdkChangeSet;

    fn deref(&self) -> &Self::Target {
        &self.changeset
    }
}

impl From<BdkChangeSet> for ChangeSet {
    fn from(changeset: BdkChangeSet) -> Self {
        ChangeSet { changeset }
    }
}

impl From<ChangeSet> for BdkChangeSet {
    fn from(changeset: ChangeSet) -> Self {
        changeset.changeset
    }
}
