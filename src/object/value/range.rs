use napi::{JsNumber, Result, Env};
use teo::prelude::{app::data::AppData, App, Range as TeoRange};

use super::teo_value_to_js_any;

#[napi(js_name = "Range")]
pub struct Range {
    pub(crate) value: TeoRange,
    pub(crate) app_data: AppData,
}

#[napi]
impl Range {

    #[napi]
    pub fn upperbond(&self, env: Env) -> Result<JsNumber> {
        let value = self.value.end.as_ref();
        let any = teo_value_to_js_any(self.app, value, &env)?;
        Ok(any.coerce_to_number()?)
    }

    #[napi]
    pub fn lowerbond(&self, env: Env) -> Result<JsNumber> {
        let value = self.value.start.as_ref();
        let any = teo_value_to_js_any(self.app, value, &env)?;
        Ok(any.coerce_to_number()?)
    }

    #[napi]
    pub fn is_closed(&self) -> bool {
        self.value.closed
    }

    #[napi]
    pub fn is_open(&self) -> bool {
        !self.value.closed
    }
}