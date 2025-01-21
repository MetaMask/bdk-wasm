//! Test suite for the Web and headless browsers.

#![cfg(all(feature = "esplora", target_arch = "wasm32"))]

extern crate wasm_bindgen_test;

use bitcoindevkit::{
    bitcoin::{EsploraClient, Wallet},
    set_panic_hook,
    types::{Address, Amount, FeeRate, KeychainKind, Network, Recipient},
};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

const ESPLORA_URL: &str = "https://mutinynet.com/api";
const STOP_GAP: usize = 5;
const PARALLEL_REQUESTS: usize = 1;
const NETWORK: Network = Network::Signet;
const SEND_ADMOUNT: u64 = 1000;
const FEE_RATE: u64 = 2;
const RECIPIENT_ADDRESS: &str = "tb1qd28npep0s8frcm3y7dxqajkcy2m40eysplyr9v";

#[wasm_bindgen_test]
async fn test_esplora_client() {
    set_panic_hook();

    let external_desc = "wpkh(tprv8ZgxMBicQKsPf6vydw7ixvsLKY79hmeXujBkGCNCApyft92yVYng2y28JpFZcneBYTTHycWSRpokhHE25GfHPBxnW5GpSm2dMWzEi9xxEyU/84'/1'/0'/0/*)#uel0vg9p";
    let internal_desc = "wpkh(tprv8ZgxMBicQKsPf6vydw7ixvsLKY79hmeXujBkGCNCApyft92yVYng2y28JpFZcneBYTTHycWSRpokhHE25GfHPBxnW5GpSm2dMWzEi9xxEyU/84'/1'/0'/1/*)#dd6w3a4e";

    let mut wallet = Wallet::create(NETWORK, external_desc.into(), internal_desc.into()).expect("wallet");
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
    web_sys::console::log_2(&"balance: ".into(), &balance.total().to_sat().into());
    assert!(balance.trusted_spendable().to_sat() > SEND_ADMOUNT);

    // Important to test that we can load the wallet from a changeset with the signing descriptors and be able to sign a transaction
    // as the changeset does not contain the private signing information.
    let mut loaded_wallet = Wallet::load(
        wallet.take_staged().unwrap(),
        Some(external_desc.into()),
        Some(internal_desc.into()),
    )
    .expect("load");
    assert_eq!(loaded_wallet.balance(), wallet.balance());

    let recipient = Address::new(RECIPIENT_ADDRESS, NETWORK).expect("recipient_address");
    let amount = Amount::from_sat(SEND_ADMOUNT);
    let mut psbt = loaded_wallet
        .build_tx(FeeRate::new(FEE_RATE), vec![Recipient::new(recipient, amount)])
        .expect("build_tx");

    let finalized = loaded_wallet.sign(&mut psbt).expect("sign");
    assert!(finalized);

    let tx = psbt.extract_tx().expect("extract_tx");
    blockchain_client.broadcast(&tx).await.expect("broadcast");

    web_sys::console::log_1(&tx.compute_txid().into());
}
