//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;

use bdk_wallet::bip39::Mnemonic;
use bitcoindevkit::{
    bitcoin::Wallet,
    seed_to_descriptor, set_panic_hook,
    types::{AddressType, ChangeSet, KeychainKind, Network},
};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

const NETWORK: Network = Network::Testnet;
const ADDRESS_TYPE: AddressType = AddressType::P2wpkh;
const MNEMONIC: &str = "journey embrace permit coil indoor stereo welcome maid movie easy clock spider tent slush bright luxury awake waste legal modify awkward answer acid goose";

#[wasm_bindgen_test]
async fn test_wallet() {
    set_panic_hook();

    let seed = Mnemonic::parse(MNEMONIC).unwrap().to_seed("");
    let descriptors = seed_to_descriptor(&seed, NETWORK, ADDRESS_TYPE).expect("seed_to_descriptor");
    let mut wallet = Wallet::create(NETWORK, descriptors.external(), descriptors.internal()).expect("wallet");

    let balance = wallet.balance();
    assert_eq!(balance.total().to_sat(), 0);

    let block_height = wallet.latest_checkpoint().height();
    assert_eq!(block_height, 0);
    assert_eq!(wallet.public_descriptor(KeychainKind::External), "wpkh([27f9035f/84'/1'/0']tpubDCkv2fHDfPg5hB6bFqJ4fNiins2Z8r5vKtD4xq5irCG2HsUXkgHYsj3gfGTdvAv41hoJeXjfxu7EBQqZMm6SVkxztKFtaaE7HuLdkuL7KNq/0/*)#wle7e0wp");
    assert_eq!(
        wallet.public_descriptor(KeychainKind::Internal),
        "wpkh([27f9035f/84'/1'/0']tpubDCkv2fHDfPg5hB6bFqJ4fNiins2Z8r5vKtD4xq5irCG2HsUXkgHYsj3gfGTdvAv41hoJeXjfxu7EBQqZMm6SVkxztKFtaaE7HuLdkuL7KNq/1/*)#ltuly67e"
    );

    let address0 = wallet.reveal_next_address(KeychainKind::External);
    assert_eq!(address0.index(), 0);

    let address1 = wallet.reveal_next_address(KeychainKind::External);
    assert_eq!(address1.index(), 1);
}

#[wasm_bindgen_test]
async fn test_changeset() {
    set_panic_hook();

    let seed = Mnemonic::parse(MNEMONIC).unwrap().to_seed("");
    let descriptors = seed_to_descriptor(&seed, NETWORK, ADDRESS_TYPE).expect("seed_to_descriptor");
    let mut wallet = Wallet::create(NETWORK, descriptors.external(), descriptors.internal()).expect("wallet");

    let mut changeset = wallet.take_staged().expect("initial_changeset");
    assert!(!changeset.is_empty());

    let changeset_from_json = ChangeSet::from_json(&changeset.to_json()).expect("changeset from_json");
    assert_eq!(changeset, changeset_from_json);

    wallet.reveal_addresses_to(KeychainKind::External, 10);

    let final_changeset = wallet.take_staged().expect("final_changeset");
    assert!(!final_changeset.is_empty());

    changeset.merge(final_changeset);
    assert!(!changeset.is_empty());
}
