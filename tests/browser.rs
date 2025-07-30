//! Test suite for the Web and headless browsers.

#![cfg(all(feature = "esplora", target_arch = "wasm32"))]

extern crate wasm_bindgen_test;

use bitcoindevkit::{
    bitcoin::{EsploraClient, Wallet},
    set_panic_hook,
    types::Network,
};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

// Only used to test that the package runs in the browser and do HTTP calls
#[wasm_bindgen_test]
async fn test_browser() {
    set_panic_hook();

    let wallet = Wallet::create(
        Network::Signet,
        "wpkh(tprv8ZgxMBicQKsPe2qpAuh1K1Hig72LCoP4JgNxZM2ZRWHZYnpuw5oHoGBsQm7Qb8mLgPpRJVn3hceWgGQRNbPD6x1pp2Qme2YFRAPeYh7vmvE/84'/1'/0'/0/*)#a6kgzlgq".into(),
         "wpkh(tprv8ZgxMBicQKsPe2qpAuh1K1Hig72LCoP4JgNxZM2ZRWHZYnpuw5oHoGBsQm7Qb8mLgPpRJVn3hceWgGQRNbPD6x1pp2Qme2YFRAPeYh7vmvE/84'/1'/0'/1/*)#vwnfl2cc".into(),
    ).expect("wallet");
    let blockchain_client = EsploraClient::new("https://mutinynet.com/api", 6).expect("esplora_client");

    let block_height = wallet.latest_checkpoint().height();
    assert_eq!(block_height, 0);

    let sync_request = wallet.start_sync_with_revealed_spks();
    let update = blockchain_client.sync(sync_request, 1).await.expect("sync");
    wallet.apply_update(update).expect("sync apply_update");

    let sync_block_height = wallet.latest_checkpoint().height();
    assert!(sync_block_height > block_height);
}
