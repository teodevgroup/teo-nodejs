use napi::{Result, JsUnknown, Env};
use teo::prelude::r#enum;

use crate::object::{js_any_to_teo_value, value::teo_value_to_js_any};

#[napi(js_name = "Enum")]
pub struct Enum {
    pub(crate) builder: r#enum::Builder,
}

#[napi]
impl Enum {

    #[napi]
    pub fn set_data(&self, key: String, value: JsUnknown, env: Env) -> Result<()> {
        self.builder.data().insert(key, js_any_to_teo_value(value, env)?);
        Ok(())
    }

    #[napi]
    pub fn data(&self, key: String, env: Env) -> Result<JsUnknown> {
        Ok(match self.builder.data().get(key.as_str()) {
            Some(object) => teo_value_to_js_any(object, &env)?,
            None => env.get_undefined()?.into_unknown(),
        })
    }
}