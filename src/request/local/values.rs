use napi::{Env, JsObject, JsUnknown, Result};
use teo::prelude::request::local_values::LocalValues as TeoLocalValues;

#[napi(js_name = "LocalValues")]
pub struct LocalValues {
    pub(crate) teo_local_values: TeoLocalValues,
}