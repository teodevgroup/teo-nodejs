use napi::bindgen_prelude::FromNapiValue;
use napi::sys::{napi_env, napi_value};
use napi::Result;

pub struct TeoUnused {}

impl FromNapiValue for TeoUnused {
    unsafe fn from_napi_value(_raw_env: napi_env, _napi_val: napi_value) -> Result<Self> {
        Ok(TeoUnused {})
    }
}