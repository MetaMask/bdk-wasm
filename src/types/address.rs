use std::{ops::Deref, str::FromStr};

use bdk_wallet::{bitcoin::AddressType as BdkAddressType, AddressInfo as BdkAddressInfo};
use bitcoin::{Address as BdkAddress, Network as BdkNetwork, ScriptBuf as BdkScriptBuf};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::result::JsResult;

use super::{KeychainKind, Network};

/// A derived address and the index it was found at.
#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct AddressInfo(BdkAddressInfo);

#[wasm_bindgen]
impl AddressInfo {
    /// Child index of this address
    #[wasm_bindgen(getter)]
    pub fn index(&self) -> u32 {
        self.0.index
    }

    /// Address
    #[wasm_bindgen(getter)]
    pub fn address(&self) -> Address {
        self.0.address.clone().into()
    }

    /// Type of keychain
    #[wasm_bindgen(getter)]
    pub fn keychain(&self) -> KeychainKind {
        self.0.keychain.into()
    }

    /// Gets the address type of the address.
    ///
    /// # Returns
    ///
    /// None if unknown, non-standard or related to the future witness version.
    #[wasm_bindgen(getter)]
    pub fn address_type(&self) -> Option<AddressType> {
        self.0.address_type().map(Into::into)
    }
}

impl Deref for AddressInfo {
    type Target = BdkAddressInfo;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<BdkAddressInfo> for AddressInfo {
    fn from(inner: BdkAddressInfo) -> Self {
        AddressInfo(inner)
    }
}

/// A Bitcoin address.
#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct Address(BdkAddress);

impl Deref for Address {
    type Target = BdkAddress;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[wasm_bindgen]
impl Address {
    pub fn from_string(address_str: &str, network: Network) -> JsResult<Self> {
        let address = BdkAddress::from_str(address_str)?.require_network(network.into())?;
        Ok(Address(address))
    }

    /// Constructs an [`Address`] from an output script (`scriptPubkey`).
    pub fn from_script(script_buf: ScriptBuf, network: Network) -> JsResult<Self> {
        let bdk_network: BdkNetwork = network.into();
        let address = BdkAddress::from_script(&script_buf, bdk_network)?;
        Ok(Address(address))
    }

    #[allow(clippy::inherent_to_string)]
    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl From<BdkAddress> for Address {
    fn from(inner: BdkAddress) -> Self {
        Address(inner)
    }
}

impl From<Address> for BdkAddress {
    fn from(address: Address) -> Self {
        address.0
    }
}

/// An owned, growable script.
///
/// `ScriptBuf` is the most common script type that has the ownership over the contents of the
/// script. It has a close relationship with its borrowed counterpart, [`Script`].
#[wasm_bindgen]
pub struct ScriptBuf(BdkScriptBuf);

impl Deref for ScriptBuf {
    type Target = BdkScriptBuf;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[wasm_bindgen]
impl ScriptBuf {
    #[allow(clippy::inherent_to_string)]
    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl From<BdkScriptBuf> for ScriptBuf {
    fn from(inner: BdkScriptBuf) -> Self {
        ScriptBuf(inner)
    }
}

impl From<ScriptBuf> for BdkScriptBuf {
    fn from(script_buf: ScriptBuf) -> Self {
        script_buf.0
    }
}

/// The different types of addresses.
#[wasm_bindgen]
#[derive(Debug)]
pub enum AddressType {
    /// Pay to pubkey hash.
    P2pkh = "p2pkh",
    /// Pay to script hash.
    P2sh = "p2sh",
    /// Pay to witness pubkey hash.
    P2wpkh = "p2wpkh",
    /// Pay to witness script hash.
    P2wsh = "p2wsh",
    /// Pay to taproot.
    P2tr = "p2tr",
}

impl From<BdkAddressType> for AddressType {
    fn from(address_type: BdkAddressType) -> Self {
        match address_type {
            BdkAddressType::P2pkh => AddressType::P2pkh,
            BdkAddressType::P2sh => AddressType::P2sh,
            BdkAddressType::P2wpkh => AddressType::P2wpkh,
            BdkAddressType::P2wsh => AddressType::P2wsh,
            BdkAddressType::P2tr => AddressType::P2tr,
            _ => panic!("Unsupported address type"),
        }
    }
}

impl From<AddressType> for BdkAddressType {
    fn from(address_type: AddressType) -> Self {
        match address_type {
            AddressType::P2pkh => BdkAddressType::P2pkh,
            AddressType::P2sh => BdkAddressType::P2sh,
            AddressType::P2wpkh => BdkAddressType::P2wpkh,
            AddressType::P2wsh => BdkAddressType::P2wsh,
            AddressType::P2tr => BdkAddressType::P2tr,
            _ => panic!("Unsupported address type"),
        }
    }
}
