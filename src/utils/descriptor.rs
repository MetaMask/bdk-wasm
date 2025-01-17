use std::str::FromStr;

use bdk_wallet::keys::ExtendedKey;
use bitcoin::bip32::{Fingerprint, Xpriv, Xpub};
use serde_wasm_bindgen::from_value;

use crate::types::{AddressType, Network, SLIP10Node};
use wasm_bindgen::prelude::{wasm_bindgen, JsError, JsValue};

use super::result::JsResult;

/// Pair of descriptors for external and internal keychains
#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct DescriptorPair {
    /// External descriptor
    external: String,
    /// Internal descriptor
    internal: String,
}

#[wasm_bindgen]
impl DescriptorPair {
    #[wasm_bindgen(constructor)]
    pub fn new(external: String, internal: String) -> Self {
        DescriptorPair { external, internal }
    }

    #[wasm_bindgen(getter)]
    pub fn internal(&self) -> String {
        self.internal.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn external(&self) -> String {
        self.external.clone()
    }
}

#[wasm_bindgen]
pub fn seed_to_descriptor(seed: &[u8], network: Network, address_type: AddressType) -> JsResult<DescriptorPair> {
    let (external, internal) = crate::bitcoin::seed_to_descriptor(seed, network.into(), address_type.into())
        .map_err(|e| JsError::new(&e.to_string()))?;

    Ok(DescriptorPair::new(
        external.0.to_string_with_secret(&external.1),
        internal.0.to_string_with_secret(&internal.1),
    ))
}

#[wasm_bindgen]
pub fn xpriv_to_descriptor(
    extended_privkey: &str,
    fingerprint: &str,
    network: Network,
    address_type: AddressType,
) -> JsResult<DescriptorPair> {
    let xprv = Xpriv::from_str(extended_privkey)?;
    let fingerprint = Fingerprint::from_hex(fingerprint)?;

    let (external, internal) =
        crate::bitcoin::xpriv_to_descriptor(xprv, fingerprint, network.into(), address_type.into())
            .map_err(|e| JsError::new(&e.to_string()))?;

    Ok(DescriptorPair::new(
        external.0.to_string_with_secret(&external.1),
        internal.0.to_string_with_secret(&internal.1),
    ))
}

#[wasm_bindgen]
pub fn xpub_to_descriptor(
    extended_pubkey: &str,
    fingerprint: &str,
    network: Network,
    address_type: AddressType,
) -> JsResult<DescriptorPair> {
    let xpub = Xpub::from_str(extended_pubkey)?;
    let fingerprint = Fingerprint::from_hex(fingerprint)?;

    let (external, internal) =
        crate::bitcoin::xpub_to_descriptor(xpub, fingerprint, network.into(), address_type.into())
            .map_err(|e| JsError::new(&e.to_string()))?;

    Ok(DescriptorPair::new(external.0.to_string(), internal.0.to_string()))
}

#[wasm_bindgen]
pub fn seed_to_xpriv(seed: &[u8], network: Network) -> JsResult<String> {
    let xprv = crate::bitcoin::seed_to_xpriv(seed, network.into()).map_err(|e| JsError::new(&e.to_string()))?;

    Ok(xprv.to_string())
}

#[wasm_bindgen]
pub fn slip10_to_extended(slip10: JsValue, network: Network) -> JsResult<String> {
    let node: SLIP10Node = from_value(slip10.clone())?;
    let extended_key =
        crate::bitcoin::slip10_to_extended(node, network.into()).map_err(|e| JsError::new(&e.to_string()))?;

    match &extended_key {
        ExtendedKey::Private(xprv) => Ok(xprv.0.to_string()),
        ExtendedKey::Public(xpub) => Ok(xpub.0.to_string()),
    }
}
