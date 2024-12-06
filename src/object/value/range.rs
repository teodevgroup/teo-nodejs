use napi::{JsNumber, Result, Env};
use teo::prelude::Range as OriginalRange;

use super::teo_value_to_js_any_no_map;

#[napi(js_name = "Range")]
pub struct Range {
    pub(crate) original: OriginalRange,
}

#[napi]
impl Range {

    #[napi(getter)]
    pub fn upperbond(&self, env: Env) -> Result<JsNumber> {
        let value = self.original.end.as_ref();
        let any = teo_value_to_js_any_no_map(value, &env)?;
        Ok(any.coerce_to_number()?)
    }

    #[napi(getter)]
    pub fn lowerbond(&self, env: Env) -> Result<JsNumber> {
        let value = self.original.start.as_ref();
        let any = teo_value_to_js_any_no_map(value, &env)?;
        Ok(any.coerce_to_number()?)
    }

    #[napi(getter)]
    pub fn is_closed(&self) -> bool {
        self.original.closed
    }

    #[napi(getter)]
    pub fn is_open(&self) -> bool {
        !self.original.closed
    }
}