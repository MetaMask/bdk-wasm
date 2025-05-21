use bdk_esplora::{
    esplora_client::{AsyncClient, Builder},
    EsploraAsyncExt,
};
use bdk_wallet::{
    chain::spk_client::{FullScanRequest as BdkFullScanRequest, SyncRequest as BdkSyncRequest},
    KeychainKind,
};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    result::JsResult,
    types::{FeeEstimates, FullScanRequest, SyncRequest, Transaction, Txid, Update},
};
use std::time::Duration;

use bdk_esplora::esplora_client::Sleeper;
use gloo_timers::future::{sleep, TimeoutFuture};

use crate::utils::SendSyncWrapper;

#[wasm_bindgen]
pub struct EsploraClient {
    client: AsyncClient<WebSleeper>,
}

#[wasm_bindgen]
impl EsploraClient {
    #[wasm_bindgen(constructor)]
    pub fn new(url: &str) -> JsResult<EsploraClient> {
        let client = Builder::new(url).build_async_with_sleeper::<WebSleeper>()?;
        Ok(EsploraClient { client })
    }

    pub async fn full_scan(
        &self,
        request: FullScanRequest,
        stop_gap: usize,
        parallel_requests: usize,
    ) -> JsResult<Update> {
        let request: BdkFullScanRequest<KeychainKind> = request.into();
        let result = self.client.full_scan(request, stop_gap, parallel_requests).await?;
        Ok(result.into())
    }

    pub async fn sync(&self, request: SyncRequest, parallel_requests: usize) -> JsResult<Update> {
        let request: BdkSyncRequest<(KeychainKind, u32)> = request.into();
        let result = self.client.sync(request, parallel_requests).await?;
        Ok(result.into())
    }

    pub async fn broadcast(&self, transaction: &Transaction) -> JsResult<()> {
        self.client.broadcast(transaction).await?;
        Ok(())
    }

    pub async fn get_fee_estimates(&self) -> JsResult<FeeEstimates> {
        let fee_estimates = self.client.get_fee_estimates().await?;
        Ok(fee_estimates.into())
    }

    pub async fn get_tx(&self, txid: Txid) -> JsResult<Option<Transaction>> {
        let tx = self.client.get_tx(&txid.into()).await?;
        Ok(tx.map(Into::into))
    }
}

#[derive(Clone)]
struct WebSleeper;

impl Sleeper for WebSleeper {
    type Sleep = SendSyncWrapper<TimeoutFuture>;

    fn sleep(dur: Duration) -> Self::Sleep {
        SendSyncWrapper(sleep(dur))
    }
}
