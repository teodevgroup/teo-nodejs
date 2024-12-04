use napi::{JsUnknown, bindgen_prelude::{Promise, FromNapiValue}, Result, Error, Status};

pub(crate) struct SendJsUnknown {
    pub(crate) inner: JsUnknown
}

impl SendJsUnknown {
    pub(crate) fn new(inner: JsUnknown) -> Self {
        Self { inner }
    }
}

unsafe impl Send for SendJsUnknown { }
unsafe impl Sync for SendJsUnknown { }

pub enum SendJsUnknownOrPromise {
    Promise(Promise<SendJsUnknownOrPromise>),
    SendJsUnknown(SendJsUnknown),
}

impl FromNapiValue for SendJsUnknownOrPromise {
    unsafe fn from_napi_value(raw_env: napi::sys::napi_env, napi_val: napi::sys::napi_value) -> napi::Result<Self> {
        let unknown = JsUnknown::from_napi_value(raw_env, napi_val)?;
        if unknown.is_promise().unwrap() {
            let promise: Promise<SendJsUnknownOrPromise> = Promise::from_napi_value(raw_env, napi_val)?;
            Ok(SendJsUnknownOrPromise::Promise(promise))
        } else {
            Ok(SendJsUnknownOrPromise::SendJsUnknown(SendJsUnknown::new(unknown)))
        }
    }
}

impl SendJsUnknownOrPromise {

    pub async fn to_send_js_unknown(self) -> Result<SendJsUnknown> {
        Ok(match self {
            SendJsUnknownOrPromise::Promise(promise) => match promise.await {
                Ok(p) => match p {
                    SendJsUnknownOrPromise::Promise(promise) => Err(Error::new(Status::Unknown, "nested promise detected"))?,
                    SendJsUnknownOrPromise::SendJsUnknown(v) => v,
                },
                Err(e) => Err(e)?,
            },
            SendJsUnknownOrPromise::SendJsUnknown(v) => v,
        })
    }

    pub async fn to_js_unknown(self) -> Result<JsUnknown> {
        Ok(match self {
            SendJsUnknownOrPromise::Promise(promise) => match promise.await {
                Ok(p) => match p {
                    SendJsUnknownOrPromise::Promise(promise) => Err(Error::new(Status::Unknown, "nested promise detected"))?,
                    SendJsUnknownOrPromise::SendJsUnknown(v) => v.inner,
                },
                Err(e) => Err(e)?,
            },
            SendJsUnknownOrPromise::SendJsUnknown(v) => v.inner,
        })
    }
}