use napi::bindgen_prelude::{FromNapiValue, Promise};
use napi::{Result, JsUnknown, Status, Error};
use napi::sys::{napi_env, napi_value};

pub enum PromiseOrIgnore {
    Promise(Promise<PromiseOrIgnore>),
    Ignore(()),
}

unsafe impl Send for PromiseOrIgnore {}
unsafe impl Sync for PromiseOrIgnore {}

impl PromiseOrIgnore {
    pub async fn to_ignore(self) -> Result<()> {
        Ok(match self {
            PromiseOrIgnore::Promise(promise) => match promise.await {
                Ok(p) => match p {
                    PromiseOrIgnore::Promise(_) => Err(Error::new(Status::Unknown, "nested promise detected"))?,
                    PromiseOrIgnore::Ignore(v) => v,
                },
                Err(e) => Err(e)?,
            },
            PromiseOrIgnore::Ignore(v) => v,
        })
    }
}

impl FromNapiValue for PromiseOrIgnore {
    unsafe fn from_napi_value(raw_env: napi_env, napi_val: napi_value) -> Result<Self> {
        let unknown = JsUnknown::from_napi_value(raw_env, napi_val)?;
        if unknown.is_promise()? {
            let promise: Promise<PromiseOrIgnore> = Promise::from_napi_value(raw_env, napi_val)?;
            Ok(PromiseOrIgnore::Promise(promise))
        } else {
            Ok(PromiseOrIgnore::Ignore(()))
        }
    }
}