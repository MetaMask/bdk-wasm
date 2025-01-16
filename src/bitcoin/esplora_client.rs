use bdk_core::spk_client::{FullScanRequest as BdkFullScanRequest, SyncRequest as BdkSyncRequest};
use bdk_esplora::{
    esplora_client::{AsyncClient, Builder},
    EsploraAsyncExt,
};
use bdk_wallet::KeychainKind;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    result::JsResult,
    types::{FullScanRequest, SyncRequest, Transaction, Update},
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
        &mut self,
        request: FullScanRequest,
        stop_gap: usize,
        parallel_requests: usize,
    ) -> JsResult<Update> {
        let request: BdkFullScanRequest<KeychainKind> = request.into();
        let result = self.client.full_scan(request, stop_gap, parallel_requests).await?;
        Ok(result.into())
    }

    pub async fn sync(&mut self, request: SyncRequest, parallel_requests: usize) -> JsResult<Update> {
        let request: BdkSyncRequest<(KeychainKind, u32)> = request.into();
        let result = self.client.sync(request, parallel_requests).await?;
        Ok(result.into())
    }

    pub async fn broadcast(&self, transaction: &Transaction) -> JsResult<()> {
        self.client.broadcast(transaction).await?;
        Ok(())
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
