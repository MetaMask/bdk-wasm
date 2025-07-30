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
    JsCast, JsError, JsValue,
};
use wasm_bindgen_futures::JsFuture;
use web_sys::js_sys::{global, Function, Promise, Reflect};

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
    client: AsyncClient<WasmSleeper>,
}

#[wasm_bindgen]
impl EsploraClient {
    #[wasm_bindgen(constructor)]
    pub fn new(url: &str, max_retries: usize) -> JsResult<EsploraClient> {
        let client = Builder::new(url)
            .max_retries(max_retries)
            .build_async_with_sleeper::<WasmSleeper>()?;
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

struct WasmSleep(JsFuture);

impl Future for WasmSleep {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        // delegate to the inner JsFuture
        Pin::new(&mut self.get_mut().0).poll(cx).map(|_| ())
    }
}

// SAFETY: Wasm is single-threaded; the value is never accessed concurrently.
unsafe impl Send for WasmSleep {}

#[derive(Clone, Copy)]
struct WasmSleeper;

impl Sleeper for WasmSleeper {
    type Sleep = WasmSleep;

    fn sleep(dur: Duration) -> Self::Sleep {
        let ms = dur.as_millis();
        let promise = Promise::new(&mut |resolve, _reject| {
            let cb = Closure::once_into_js(move || resolve.call0(&JsValue::NULL).unwrap()).unchecked_into::<Function>();

            // globalThis.setTimeout(cb, ms);
            let g = global();
            let set_timeout = Reflect::get(&g, &JsValue::from_str("setTimeout"))
                .unwrap_or_else(|_| JsError::new("setTimeout not found").into())
                .unchecked_into::<Function>();

            set_timeout.call2(&g, &cb, &JsValue::from_f64(ms as f64)).unwrap();
        });

        WasmSleep(JsFuture::from(promise))
    }
}
