use napi::{JsUnknown, bindgen_prelude::{Promise, FromNapiValue}, Result};

pub enum JsUnknownOrPromise {
    Promise(JsUnknown),
    JsUnknown(JsUnknown),
}

impl FromNapiValue for JsUnknownOrPromise {
    unsafe fn from_napi_value(raw_env: napi::sys::napi_env, napi_val: napi::sys::napi_value) -> napi::Result<Self> {
        let unknown = JsUnknown::from_napi_value(raw_env, napi_val)?;
        if unknown.is_promise()? {
            //let promise: Promise<JsUnknown> = Promise::from_napi_value(raw_env, napi_val)?;
            Ok(JsUnknownOrPromise::Promise(unknown))
        } else {
            Ok(JsUnknownOrPromise::JsUnknown(unknown))
        }
    }
}

impl JsUnknownOrPromise {

    pub fn to_js_unknown(self) -> Result<JsUnknown> {
        match self {
            JsUnknownOrPromise::Promise(promise) => Ok(promise),
            JsUnknownOrPromise::JsUnknown(v) => Ok(v),
        }
    }

    // pub async fn resolve_promise_to_js_unknown(self) -> Result<JsUnknown> {
    //     match self {
    //         JsUnknownOrPromise::Promise(promise) => promise.await,
    //         JsUnknownOrPromise::JsUnknown(v) => Ok(v),
    //     }
    // }
}

unsafe impl Send for JsUnknownOrPromise { }
unsafe impl Sync for JsUnknownOrPromise { }
