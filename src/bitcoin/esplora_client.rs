use bdk_esplora::{
    esplora_client::{AsyncClient, Builder},
    EsploraAsyncExt,
};
use bdk_wallet::{
    chain::spk_client::{FullScanRequest as BdkFullScanRequest, SyncRequest as BdkSyncRequest},
    KeychainKind,
};
use wasm_bindgen::{
    prelude::{wasm_bindgen, Closure},
    JsCast, JsValue,
};
use wasm_bindgen_futures::JsFuture;
use web_sys::js_sys::{Function, Promise};

use crate::{
    result::JsResult,
    types::{FeeEstimates, FullScanRequest, SyncRequest, Transaction, Txid, Update},
};
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};

use bdk_esplora::esplora_client::Sleeper;

#[wasm_bindgen]
pub struct EsploraClient {
    client: AsyncClient<WebSleeper>,
}

#[wasm_bindgen]
impl EsploraClient {
    #[wasm_bindgen(constructor)]
    pub fn new(url: &str, max_retries: usize) -> JsResult<EsploraClient> {
        let client = Builder::new(url)
            .max_retries(max_retries)
            .build_async_with_sleeper::<WebSleeper>()?;
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

struct WebSleep(JsFuture);

impl Future for WebSleep {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        // delegate to the inner JsFuture
        Pin::new(&mut self.get_mut().0).poll(cx).map(|_| ())
    }
}

// SAFETY: Wasm is single-threaded; the value is never accessed concurrently.
unsafe impl Send for WebSleep {}

#[derive(Clone, Copy)]
struct WebSleeper;

impl Sleeper for WebSleeper {
    type Sleep = WebSleep;

    fn sleep(dur: Duration) -> Self::Sleep {
        let ms = dur.as_millis() as i32;
        let promise = Promise::new(&mut |resolve, _reject| {
            let cb = Closure::once_into_js(move || resolve.call0(&JsValue::NULL).unwrap());
            web_sys::window()
                .unwrap()
                .set_timeout_with_callback_and_timeout_and_arguments_0(cb.unchecked_ref::<Function>(), ms)
                .unwrap();
        });
        WebSleep(JsFuture::from(promise))
    }
}
