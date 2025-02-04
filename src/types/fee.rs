use std::{collections::HashMap, ops::Deref};

use wasm_bindgen::prelude::wasm_bindgen;

/// Map where the key is the confirmation target (in number of blocks) and the value is the estimated feerate (in sat/vB).
#[wasm_bindgen]
#[derive(Debug)]
pub struct FeeEstimates(HashMap<u16, f64>);

impl Deref for FeeEstimates {
    type Target = HashMap<u16, f64>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[wasm_bindgen]
impl FeeEstimates {
    /// Returns the feerate (in sat/vB) or undefined.
    /// Available confirmation targets are 1-25, 144, 504 and 1008 blocks.
    #[wasm_bindgen(getter)]
    pub fn get(&self, k: u16) -> Option<f64> {
        self.0.get(&k).copied()
    }
}

impl From<HashMap<u16, f64>> for FeeEstimates {
    fn from(inner: HashMap<u16, f64>) -> Self {
        FeeEstimates(inner)
    }
}

impl From<FeeEstimates> for HashMap<u16, f64> {
    fn from(fee_estimates: FeeEstimates) -> Self {
        fee_estimates.0
    }
}
