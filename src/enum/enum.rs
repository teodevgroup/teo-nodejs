use napi::{Result, JsUnknown, Env};
use teo::prelude::Enum as TeoEnum;

use crate::object::{js_any_to_teo_value, value::teo_value_to_js_any};

#[napi(js_name = "Enum")]
pub struct Enum {
    pub(crate) teo_enum: &'static mut TeoEnum,
}

#[napi]
impl Enum {

    #[napi]
    pub fn set_data(&mut self, key: String, value: JsUnknown, env: Env) -> Result<()> {
        self.teo_enum.data.insert(key, js_any_to_teo_value(value, env)?);
        Ok(())
    }

    #[napi]
    pub fn data(&mut self, key: String, env: Env) -> Result<JsUnknown> {
        Ok(match self.teo_enum.data.get(key.as_str()) {
            Some(object) => teo_value_to_js_any(object, &env)?,
            None => env.get_undefined()?.into_unknown(),
        })
    }
}