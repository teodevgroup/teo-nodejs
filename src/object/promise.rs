use napi::bindgen_prelude::{FromNapiValue, Promise};
use napi::{Result, Env, JsUnknown, Status, Error};
use napi::sys::{napi_env, napi_value};
use teo::prelude::Value as TeoValue;
use crate::object::js_any_to_teo_value;

pub enum TeoValueOrPromise {
    Promise(Promise<TeoValueOrPromise>),
    TeoValue(TeoValue),
}

unsafe impl Send for TeoValueOrPromise {}
unsafe impl Sync for TeoValueOrPromise {}

impl TeoValueOrPromise {
    pub async fn to_teo_value(self) -> Result<TeoValue> {
        Ok(match self {
            TeoValueOrPromise::Promise(promise) => match promise.await {
                Ok(p) => match p {
                    TeoValueOrPromise::Promise(promise) => Err(Error::new(Status::Unknown, "nested promise detected"))?,
                    TeoValueOrPromise::TeoValue(v) => v,
                },
                Err(e) => Err(e)?,
            },
            TeoValueOrPromise::TeoValue(v) => v,
        })
    }
}

impl FromNapiValue for TeoValueOrPromise {
    unsafe fn from_napi_value(raw_env: napi_env, napi_val: napi_value) -> Result<Self> {
        let env = Env::from_raw(raw_env);
        let unknown = JsUnknown::from_napi_value(raw_env, napi_val).unwrap();
        if unknown.is_promise().unwrap() {
            let promise: Promise<TeoValueOrPromise> = Promise::from_napi_value(raw_env, napi_val).unwrap();
            Ok(TeoValueOrPromise::Promise(promise))
        } else {
            Ok(TeoValueOrPromise::TeoValue(js_any_to_teo_value(unknown, env)?))
        }
    }
}