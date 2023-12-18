use napi::bindgen_prelude::{FromNapiValue, Promise};
use napi::{Env, JsUnknown};
use napi::sys::{napi_env, napi_value};
use crate::object::js_unknown_to_teo_value;
use teo::prelude::object::{Object as TeoObject, ObjectInner};
use teo::prelude::{Value as TeoValue};

pub enum TeoObjectOrPromise {
    Promise(Promise<TeoObjectOrPromise>),
    TeoObject(TeoObject),
}

unsafe impl Send for TeoObjectOrPromise {}
unsafe impl Sync for TeoObjectOrPromise {}

impl TeoObjectOrPromise {
    pub async fn to_teo_object(self) -> TeoObject {
        match self {
            TeoObjectOrPromise::Promise(promise) => match promise.await {
                Ok(p) => match p {
                    TeoObjectOrPromise::Promise(p) => p.await?.to_teo_object(),
                    TeoObjectOrPromise::TeoObject(v) => v,
                },
                Err(e) => TeoValue::String(e.reason.clone()),
            },
            TeoObjectOrPromise::TeoObject(v) => v,
        }
    }
}

impl FromNapiValue for TeoObjectOrPromise {
    unsafe fn from_napi_value(raw_env: napi_env, napi_val: napi_value) -> napi::Result<Self> {
        let env = Env::from_raw(raw_env);
        let unknown = JsUnknown::from_napi_value(raw_env, napi_val).unwrap();
        if unknown.is_promise().unwrap() {
            let promise: Promise<TeoObjectOrPromise> = Promise::from_napi_value(raw_env, napi_val).unwrap();
            Ok(TeoObjectOrPromise::Promise(promise))
        } else {
            Ok(TeoObjectOrPromise::TeoObject(js_unknown_to_teo_value(unknown, env)))
        }
    }
}