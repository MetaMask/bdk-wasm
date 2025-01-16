//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;

use bdk_wallet::bip39::Mnemonic;
use bitcoindevkit::{
    seed_to_descriptor, seed_to_xpriv, set_panic_hook,
    types::{AddressType, Network},
    xpriv_to_descriptor, xpub_to_descriptor,
};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

const NETWORK: Network = Network::Testnet;
const ADDRESS_TYPE: AddressType = AddressType::P2wpkh;
const MNEMONIC: &str = "journey embrace permit coil indoor stereo welcome maid movie easy clock spider tent slush bright luxury awake waste legal modify awkward answer acid goose";

#[wasm_bindgen_test]
async fn test_seed_to_xpriv() {
    set_panic_hook();

    let seed = Mnemonic::parse(MNEMONIC).unwrap().to_seed("");
    let xprv = seed_to_xpriv(&seed, NETWORK).expect("seed_to_xpriv");

    assert_eq!(
        xprv,
        "tprv8ZgxMBicQKsPf6vydw7ixvsLKY79hmeXujBkGCNCApyft92yVYng2y28JpFZcneBYTTHycWSRpokhHE25GfHPBxnW5GpSm2dMWzEi9xxEyU"
    );
}

#[wasm_bindgen_test]
async fn test_seed_to_descriptor() {
    set_panic_hook();

    let seed = Mnemonic::parse(MNEMONIC).unwrap().to_seed("");
    let descriptors = seed_to_descriptor(&seed, NETWORK, ADDRESS_TYPE).expect("seed_to_descriptor");

    assert_eq!(
        descriptors.external(),
        "wpkh(tprv8ZgxMBicQKsPf6vydw7ixvsLKY79hmeXujBkGCNCApyft92yVYng2y28JpFZcneBYTTHycWSRpokhHE25GfHPBxnW5GpSm2dMWzEi9xxEyU/84'/1'/0'/0/*)#uel0vg9p"
    );
    assert_eq!(
        descriptors.internal(),
        "wpkh(tprv8ZgxMBicQKsPf6vydw7ixvsLKY79hmeXujBkGCNCApyft92yVYng2y28JpFZcneBYTTHycWSRpokhHE25GfHPBxnW5GpSm2dMWzEi9xxEyU/84'/1'/0'/1/*)#dd6w3a4e"
    );
}

#[wasm_bindgen_test]
async fn test_xpriv_to_descriptor() {
    set_panic_hook();
    let xpriv = "tprv8g4stFEyX1zQoi4oNBdUFy4cDqWcyWu1kacHgK3RRvTdTPDm8HTxhERpV9JLTct69h4479xKJXm85SYkFZ4eMUsru5MdUNkeouuzbivKAJp";
    let fingerprint = "27f9035f";

    let descriptors = xpriv_to_descriptor(xpriv, fingerprint, NETWORK, ADDRESS_TYPE).expect("xpriv_to_descriptor");

    assert_eq!(
        descriptors.external(),
        "wpkh([27f9035f/84'/1'/0']tprv8g4stFEyX1zQoi4oNBdUFy4cDqWcyWu1kacHgK3RRvTdTPDm8HTxhERpV9JLTct69h4479xKJXm85SYkFZ4eMUsru5MdUNkeouuzbivKAJp/0/*)#sx5quhf7"
    );
    assert_eq!(
        descriptors.internal(),
        "wpkh([27f9035f/84'/1'/0']tprv8g4stFEyX1zQoi4oNBdUFy4cDqWcyWu1kacHgK3RRvTdTPDm8HTxhERpV9JLTct69h4479xKJXm85SYkFZ4eMUsru5MdUNkeouuzbivKAJp/1/*)#pj3ppzex"
    );
}

#[wasm_bindgen_test]
async fn test_xpub_to_descriptor() {
    set_panic_hook();
    let xpub = "tpubDCkv2fHDfPg5hB6bFqJ4fNiins2Z8r5vKtD4xq5irCG2HsUXkgHYsj3gfGTdvAv41hoJeXjfxu7EBQqZMm6SVkxztKFtaaE7HuLdkuL7KNq";
    let fingerprint = "27f9035f";

    let descriptors = xpub_to_descriptor(xpub, fingerprint, NETWORK, ADDRESS_TYPE).expect("xpub_to_descriptor");

    assert_eq!(
        descriptors.external(),
        "wpkh([27f9035f/84'/1'/0']tpubDCkv2fHDfPg5hB6bFqJ4fNiins2Z8r5vKtD4xq5irCG2HsUXkgHYsj3gfGTdvAv41hoJeXjfxu7EBQqZMm6SVkxztKFtaaE7HuLdkuL7KNq/0/*)#wle7e0wp"
    );
    assert_eq!(
        descriptors.internal(),
        "wpkh([27f9035f/84'/1'/0']tpubDCkv2fHDfPg5hB6bFqJ4fNiins2Z8r5vKtD4xq5irCG2HsUXkgHYsj3gfGTdvAv41hoJeXjfxu7EBQqZMm6SVkxztKFtaaE7HuLdkuL7KNq/1/*)#ltuly67e"
    );
}
