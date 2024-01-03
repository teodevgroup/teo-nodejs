use napi::bindgen_prelude::{FromNapiValue, Promise, FromNapiRef};
use napi::{Result, JsUnknown, Status, Error, Env};
use napi::sys::{napi_env, napi_value};
use teo::prelude::Response as TeoResponse;
use crate::console::console_log;
use crate::response::Response;

pub enum ResponseOrPromise {
    Promise(Promise<ResponseOrPromise>),
    TeoResponse(TeoResponse),
}

unsafe impl Send for ResponseOrPromise {}
unsafe impl Sync for ResponseOrPromise {}

impl ResponseOrPromise {
    pub async fn to_teo_response(self) -> Result<TeoResponse> {
        Ok(match self {
            ResponseOrPromise::Promise(promise) => match promise.await {
                Ok(p) => match p {
                    ResponseOrPromise::Promise(_) => Err(Error::new(Status::Unknown, "nested promise detected"))?,
                    ResponseOrPromise::TeoResponse(v) => v,
                },
                Err(e) => Err(e)?,
            },
            ResponseOrPromise::TeoResponse(v) => v,
        })
    }
}

impl FromNapiValue for ResponseOrPromise {
    unsafe fn from_napi_value(raw_env: napi_env, napi_val: napi_value) -> Result<Self> {
        let unknown = JsUnknown::from_napi_value(raw_env, napi_val)?;
        if unknown.is_promise()? {
            let promise: Promise<ResponseOrPromise> = Promise::from_napi_value(raw_env, napi_val)?;
            Ok(ResponseOrPromise::Promise(promise))
        } else {
            let response = Response::from_napi_ref(raw_env, napi_val)?;
            Ok(ResponseOrPromise::TeoResponse(response.teo_response.clone()))
        }
    }
}