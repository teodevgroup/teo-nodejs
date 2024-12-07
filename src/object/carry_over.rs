use std::ptr;
use napi::{bindgen_prelude::{FromNapiValue, ToNapiValue}, sys::{napi_create_reference, napi_get_reference_value}, Result};

pub struct CarryOverValue {
    reference: napi::sys::napi_ref,
}

impl FromNapiValue for CarryOverValue {
    unsafe fn from_napi_value(raw_env: napi::sys::napi_env, napi_val: napi::sys::napi_value) -> napi::Result<Self> {
        let mut reference = ptr::null_mut();
        let _ = napi_create_reference(raw_env, napi_val, 1, &mut reference);
        Ok(CarryOverValue { reference })
    }
}

impl ToNapiValue for CarryOverValue {
    unsafe fn to_napi_value(env: napi::sys::napi_env, val: Self) -> Result<napi::sys::napi_value> {
        let mut value = ptr::null_mut();
        let _ = napi_get_reference_value(env, val.reference, &mut value);
        Ok(value)
    }
}

unsafe impl Send for CarryOverValue { }
unsafe impl Sync for CarryOverValue { }
