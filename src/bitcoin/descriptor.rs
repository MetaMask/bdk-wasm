use anyhow::{anyhow, Error};
use bdk_wallet::{
    bitcoin::{
        bip32::{ChainCode, Fingerprint, Xpriv, Xpub},
        hex::FromHex,
        secp256k1::{PublicKey, SecretKey},
        AddressType, Network,
    },
    keys::{DerivableKey, ExtendedKey},
    template::{
        Bip44, Bip44Public, Bip49, Bip49Public, Bip84, Bip84Public, Bip86, Bip86Public, DescriptorTemplate,
        DescriptorTemplateOut,
    },
    KeychainKind,
};

use crate::types::SLIP10Node;

pub fn seed_to_descriptor(
    seed: &[u8],
    network: Network,
    address_type: AddressType,
) -> Result<(DescriptorTemplateOut, DescriptorTemplateOut), Error> {
    let xprv = seed_to_xpriv(seed, network)?;

    match address_type {
        AddressType::P2pkh => build_xpriv_descriptor(Bip44, xprv, network),
        AddressType::P2sh => build_xpriv_descriptor(Bip49, xprv, network),
        AddressType::P2wpkh => build_xpriv_descriptor(Bip84, xprv, network),
        AddressType::P2tr => build_xpriv_descriptor(Bip86, xprv, network),
        _ => Err(anyhow!("Unsupported address type")),
    }
}

pub fn xpriv_to_descriptor(
    xprv: Xpriv,
    fingerprint: Fingerprint,
    network: Network,
    address_type: AddressType,
) -> Result<(DescriptorTemplateOut, DescriptorTemplateOut), Error> {
    match address_type {
        AddressType::P2pkh => build_descriptor(Bip44Public, xprv, fingerprint, network),
        AddressType::P2sh => build_descriptor(Bip49Public, xprv, fingerprint, network),
        AddressType::P2wpkh => build_descriptor(Bip84Public, xprv, fingerprint, network),
        AddressType::P2tr => build_descriptor(Bip86Public, xprv, fingerprint, network),
        _ => Err(anyhow!("Unsupported address type")),
    }
}

pub fn xpub_to_descriptor(
    xpub: Xpub,
    fingerprint: Fingerprint,
    network: Network,
    address_type: AddressType,
) -> Result<(DescriptorTemplateOut, DescriptorTemplateOut), Error> {
    match address_type {
        AddressType::P2pkh => build_descriptor(Bip44Public, xpub, fingerprint, network),
        AddressType::P2sh => build_descriptor(Bip49Public, xpub, fingerprint, network),
        AddressType::P2wpkh => build_descriptor(Bip84Public, xpub, fingerprint, network),
        AddressType::P2tr => build_descriptor(Bip86Public, xpub, fingerprint, network),
        _ => Err(anyhow!("Unsupported address type")),
    }
}

pub fn slip10_to_extended(node: SLIP10Node, network: Network) -> Result<ExtendedKey, anyhow::Error> {
    let parent_fingerprint: Fingerprint = node.parent_fingerprint.to_be_bytes().into();
    let chain_code = ChainCode::from_hex(strip_0x_prefix(&node.chain_code))?;

    match node.private_key {
        Some(priv_key) => {
            let priv_key_vec = Vec::from_hex(strip_0x_prefix(&priv_key))?;
            let private_key = SecretKey::from_slice(&priv_key_vec)?;

            let xpriv = Xpriv {
                network: network.into(),
                depth: node.depth,
                parent_fingerprint,
                child_number: node.index.into(),
                chain_code,
                private_key,
            };

            Ok(xpriv.into())
        }
        None => {
            let pubkey_vec = Vec::from_hex(strip_0x_prefix(&node.public_key))?;
            let public_key = PublicKey::from_slice(&pubkey_vec)?;

            let xpub = Xpub {
                network: network.into(),
                depth: node.depth,
                parent_fingerprint,
                child_number: node.index.into(),
                chain_code,
                public_key,
            };

            Ok(xpub.into())
        }
    }
}

pub fn seed_to_xpriv(seed: &[u8], network: Network) -> Result<Xpriv, Error> {
    let xprv = Xpriv::new_master(network, seed)?;
    Ok(xprv)
}

fn build_xpriv_descriptor<T>(
    constructor: impl Fn(Xpriv, KeychainKind) -> T,
    xprv: Xpriv,
    network: Network,
) -> Result<(DescriptorTemplateOut, DescriptorTemplateOut), Error>
where
    T: DescriptorTemplate,
{
    let ext_template = constructor(xprv, KeychainKind::External);
    let int_template = constructor(xprv, KeychainKind::Internal);

    let ext = ext_template.build(network)?;
    let int = int_template.build(network)?;

    ext.0.sanity_check()?;
    int.0.sanity_check()?;

    Ok((ext, int))
}

fn build_descriptor<T, K>(
    constructor: impl Fn(T, Fingerprint, KeychainKind) -> K,
    key: T,
    fingerprint: Fingerprint,
    network: Network,
) -> Result<(DescriptorTemplateOut, DescriptorTemplateOut), Error>
where
    T: DerivableKey + Clone,
    K: DescriptorTemplate,
{
    let ext_template = constructor(key.clone(), fingerprint, KeychainKind::External);
    let int_template = constructor(key, fingerprint, KeychainKind::Internal);

    let ext = ext_template.build(network)?;
    let int = int_template.build(network)?;

    ext.0.sanity_check()?;
    int.0.sanity_check()?;

    Ok((ext, int))
}

fn strip_0x_prefix(s: &str) -> &str {
    if s.starts_with("0x") || s.starts_with("0X") {
        &s[2..]
    } else {
        s
    }
}
