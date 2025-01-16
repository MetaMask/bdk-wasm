//! Test suite for the Web and headless browsers.

#![cfg(all(feature = "esplora", target_arch = "wasm32"))]

extern crate wasm_bindgen_test;

use bitcoindevkit::{
    bitcoin::{EsploraClient, Wallet},
    set_panic_hook,
    types::{DescriptorPair, KeychainKind, Network},
};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

const STOP_GAP: usize = 5;
const PARALLEL_REQUESTS: usize = 1;
const NETWORK: Network = Network::Signet;
const EXTERNAL_DESC: &str = "wpkh([aafa6322/84'/1'/0']tpubDCfvzhCuifJtWDVdrBcPvZU7U5uyixL7QULk8hXA7KjqiNnry9Te1nwm7yStqenPCQhy5MwzxKkLBD2GmKNgvMYqXgo53iYqQ7Vu4vQbN2N/0/*)#mlua264t";
const INTERNAL_DESC: &str = "wpkh([aafa6322/84'/1'/0']tpubDCfvzhCuifJtWDVdrBcPvZU7U5uyixL7QULk8hXA7KjqiNnry9Te1nwm7yStqenPCQhy5MwzxKkLBD2GmKNgvMYqXgo53iYqQ7Vu4vQbN2N/1/*)#2teuh09n";

#[wasm_bindgen_test]
async fn test_esplora_client() {
    set_panic_hook();

    let esplora_url = match NETWORK {
        Network::Bitcoin => "https://blockstream.info/api",
        Network::Testnet => "https://blockstream.info/testnet/api",
        Network::Testnet4 => "https://blockstream.info/testnet/api",
        Network::Signet => "https://mutinynet.com/api",
        Network::Regtest => "https://localhost:3000",
        _ => panic!("unsupported network"),
    };

    let mut wallet =
        Wallet::create(NETWORK, DescriptorPair::new(EXTERNAL_DESC.into(), INTERNAL_DESC.into())).expect("wallet");
    let mut blockchain_client = EsploraClient::new(esplora_url).expect("esplora_client");

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
