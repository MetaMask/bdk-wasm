//! Test suite for the Web and headless browsers.

#![cfg(all(feature = "esplora", target_arch = "wasm32"))]

extern crate wasm_bindgen_test;

use bdk_wallet::bip39::Mnemonic;
use bitcoindevkit::{
    bitcoin::{EsploraClient, Wallet},
    seed_to_descriptor, set_panic_hook,
    types::{Address, AddressType, Amount, DescriptorPair, FeeRate, KeychainKind, Network, Recipient},
};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

const ESPLORA_URL: &str = "https://mutinynet.com/api";
const STOP_GAP: usize = 5;
const PARALLEL_REQUESTS: usize = 1;
const NETWORK: Network = Network::Signet;
const EXTERNAL_DESC: &str = "wpkh([aafa6322/84'/1'/0']tpubDCfvzhCuifJtWDVdrBcPvZU7U5uyixL7QULk8hXA7KjqiNnry9Te1nwm7yStqenPCQhy5MwzxKkLBD2GmKNgvMYqXgo53iYqQ7Vu4vQbN2N/0/*)#mlua264t";
const INTERNAL_DESC: &str = "wpkh([aafa6322/84'/1'/0']tpubDCfvzhCuifJtWDVdrBcPvZU7U5uyixL7QULk8hXA7KjqiNnry9Te1nwm7yStqenPCQhy5MwzxKkLBD2GmKNgvMYqXgo53iYqQ7Vu4vQbN2N/1/*)#2teuh09n";
const RECIPIENT_ADDRESS: &str = "tb1qd28npep0s8frcm3y7dxqajkcy2m40eysplyr9v";
const SEND_AMOUNT: u64 = 10000;

#[wasm_bindgen_test]
async fn test_esplora_client() {
    set_panic_hook();

    let mut wallet =
        Wallet::create(NETWORK, DescriptorPair::new(EXTERNAL_DESC.into(), INTERNAL_DESC.into())).expect("wallet");
    let mut blockchain_client = EsploraClient::new(ESPLORA_URL).expect("esplora_client");

    let block_height = wallet.latest_checkpoint().height();
    assert_eq!(block_height, 0);

    wallet.reveal_addresses_to(KeychainKind::External, 5);

    let sync_request = wallet.start_sync_with_revealed_spks();
    let update = blockchain_client
        .sync(sync_request, PARALLEL_REQUESTS)
        .await
        .expect("sync");
    wallet.apply_update(update).expect("sync apply_update");

    let sync_block_height = wallet.latest_checkpoint().height();
    assert!(sync_block_height > block_height);

    let full_scan_request = wallet.start_full_scan();
    let update = blockchain_client
        .full_scan(full_scan_request, STOP_GAP, PARALLEL_REQUESTS)
        .await
        .expect("full_scan");
    wallet.apply_update(update).expect("full_scan apply_update");

    let balance = wallet.balance();
    assert!(balance.total().to_sat() > 0);

    let loaded_wallet = Wallet::load(wallet.take_staged().unwrap()).expect("load");
    assert_eq!(loaded_wallet.balance(), wallet.balance());
}

#[wasm_bindgen_test]
async fn test_send() {
    set_panic_hook();

    let seed = Mnemonic::parse("journey embrace permit coil indoor stereo welcome maid movie easy clock spider tent slush bright luxury awake waste legal modify awkward answer acid goose")
        .unwrap()
        .to_seed("");
    let descriptors = seed_to_descriptor(&seed, NETWORK, AddressType::P2wpkh).expect("seed_to_descriptor");
    let mut wallet = Wallet::create(NETWORK, descriptors).expect("wallet");
    let mut blockchain_client = EsploraClient::new(ESPLORA_URL).expect("esplora_client");

    let full_scan_request = wallet.start_full_scan();
    let update = blockchain_client
        .full_scan(full_scan_request, STOP_GAP, PARALLEL_REQUESTS)
        .await
        .expect("full_scan");
    wallet.apply_update(update).expect("full_scan apply_update");

    let balance = wallet.balance();
    assert!(balance.total().to_sat() > SEND_AMOUNT);
    web_sys::console::log_2(&"Balance: ".into(), &balance.total().to_btc().into());

    let recipient = Address::new(RECIPIENT_ADDRESS, NETWORK).expect("recipient_address");
    let amount = Amount::from_sat(SEND_AMOUNT);
    let mut psbt = wallet
        .build_tx(FeeRate::new(2), vec![Recipient::new(recipient, amount)])
        .expect("build_tx");

    assert!(wallet.sign(&mut psbt).expect("sign"));

    let tx = psbt.extract_tx().expect("extract_tx");
    blockchain_client.broadcast(&tx).await.expect("broadcast");

    web_sys::console::log_1(&tx.compute_txid().into());
}
