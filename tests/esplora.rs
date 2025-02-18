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
const CONFIRMATION_TARGET: u16 = 2;

#[wasm_bindgen_test]
async fn test_esplora_client() {
    set_panic_hook();

    let external_desc = "wpkh(tprv8ZgxMBicQKsPf6vydw7ixvsLKY79hmeXujBkGCNCApyft92yVYng2y28JpFZcneBYTTHycWSRpokhHE25GfHPBxnW5GpSm2dMWzEi9xxEyU/84'/1'/0'/0/*)#uel0vg9p";
    let internal_desc = "wpkh(tprv8ZgxMBicQKsPf6vydw7ixvsLKY79hmeXujBkGCNCApyft92yVYng2y28JpFZcneBYTTHycWSRpokhHE25GfHPBxnW5GpSm2dMWzEi9xxEyU/84'/1'/0'/1/*)#dd6w3a4e";

    let wallet = Wallet::create(NETWORK, external_desc.into(), internal_desc.into()).expect("wallet");
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
    assert!(balance.trusted_spendable().to_sat() > SEND_ADMOUNT);

    // Important to test that we can load the wallet from a changeset with the signing descriptors and be able to sign a transaction
    // as the changeset does not contain the private signing information.
    let loaded_wallet = Wallet::load(
        wallet.take_staged().unwrap(),
        Some(external_desc.into()),
        Some(internal_desc.into()),
    )
    .expect("load");
    assert_eq!(loaded_wallet.balance(), wallet.balance());

    let initial_derivation_index = wallet.derivation_index(KeychainKind::Internal).unwrap();

    let fees = blockchain_client.get_fee_estimates().await.expect("get_fee_estimates");
    let recipient = Address::from_string(RECIPIENT_ADDRESS, NETWORK).expect("recipient_address");
    let amount = Amount::from_sat(SEND_ADMOUNT);
    let fee_rate = fees.get(CONFIRMATION_TARGET).expect("fee_estimation");
    let mut psbt = loaded_wallet
        .build_tx()
        .fee_rate(FeeRate::new(fee_rate as u64))
        .add_recipient(Recipient::new(recipient, amount))
        .finish()
        .expect("build_tx");

    let fee = psbt.fee().expect("psbt_fee");
    assert!(fee.to_sat() > 100); // We cannot know the exact fees

    let finalized = loaded_wallet.sign(&mut psbt).expect("sign");
    assert!(finalized);

    let tx = psbt.extract_tx().expect("extract_tx");
    blockchain_client.broadcast(&tx).await.expect("broadcast");

    // Assert that we are aware of newly created addresses that were revealed during PSBT creation
    let current_derivation_index = loaded_wallet.derivation_index(KeychainKind::Internal).unwrap();
    assert!(initial_derivation_index < current_derivation_index);

    let fetched_tx = blockchain_client.get_tx(tx.compute_txid()).await.expect("get_tx");
    assert!(fetched_tx.is_some())
}

#[wasm_bindgen_test]
async fn test_drain() {
    set_panic_hook();

    let external_desc = "wpkh(tprv8ZgxMBicQKsPf6vydw7ixvsLKY79hmeXujBkGCNCApyft92yVYng2y28JpFZcneBYTTHycWSRpokhHE25GfHPBxnW5GpSm2dMWzEi9xxEyU/84'/1'/0'/0/*)#uel0vg9p";
    let internal_desc = "wpkh(tprv8ZgxMBicQKsPf6vydw7ixvsLKY79hmeXujBkGCNCApyft92yVYng2y28JpFZcneBYTTHycWSRpokhHE25GfHPBxnW5GpSm2dMWzEi9xxEyU/84'/1'/0'/1/*)#dd6w3a4e";

    let wallet = Wallet::create(NETWORK, external_desc.into(), internal_desc.into()).expect("wallet");
    let mut blockchain_client = EsploraClient::new(ESPLORA_URL).expect("esplora_client");

    let full_scan_request = wallet.start_full_scan();
    let update = blockchain_client
        .full_scan(full_scan_request, STOP_GAP, PARALLEL_REQUESTS)
        .await
        .expect("full_scan");
    wallet.apply_update(update).expect("full_scan apply_update");

    // No need to test actual values as we are just wrapping BDK and assume the underlying package is computing fees properly
    let recipient = Address::from_string(RECIPIENT_ADDRESS, NETWORK).expect("recipient_address");
    let psbt = wallet
        .build_tx()
        .drain_wallet()
        .fee_rate(FeeRate::new(FEE_RATE))
        .drain_to(recipient)
        .finish()
        .expect("drain_to");
    assert!(psbt.fee_amount().is_some());
    assert!(psbt.fee_rate().is_some());
}

#[wasm_bindgen_test]
async fn test_unspendable() {
    set_panic_hook();

    let external_desc = "wpkh(tprv8ZgxMBicQKsPf6vydw7ixvsLKY79hmeXujBkGCNCApyft92yVYng2y28JpFZcneBYTTHycWSRpokhHE25GfHPBxnW5GpSm2dMWzEi9xxEyU/84'/1'/0'/0/*)#uel0vg9p";
    let internal_desc = "wpkh(tprv8ZgxMBicQKsPf6vydw7ixvsLKY79hmeXujBkGCNCApyft92yVYng2y28JpFZcneBYTTHycWSRpokhHE25GfHPBxnW5GpSm2dMWzEi9xxEyU/84'/1'/0'/1/*)#dd6w3a4e";

    let wallet = Wallet::create(NETWORK, external_desc.into(), internal_desc.into()).expect("wallet");
    let mut blockchain_client = EsploraClient::new(ESPLORA_URL).expect("esplora_client");

    let full_scan_request = wallet.start_full_scan();
    let update = blockchain_client
        .full_scan(full_scan_request, STOP_GAP, PARALLEL_REQUESTS)
        .await
        .expect("full_scan");
    wallet.apply_update(update).expect("full_scan apply_update");

    let utxos = wallet.list_unspent();
    assert_eq!(utxos.len(), 1, "should have only 1 UTXO");

    let result = wallet
        .build_tx()
        .drain_wallet()
        .unspendable(vec![utxos[0].outpoint()])
        .finish();
    assert!(result.is_err());
}
