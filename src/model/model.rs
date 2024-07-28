use napi::{Result, JsUnknown, Env};
use teo::prelude::model;

use crate::object::{js_any_to_teo_value, value::teo_value_to_js_any};

#[napi(js_name = "Model")]
pub struct Model {
    pub(crate) builder: model::Builder,
}

#[napi]
impl Model {

    #[napi]
    pub fn set_data(&self, key: String, value: JsUnknown, env: Env) -> Result<()> {
        self.builder.data().insert(key, js_any_to_teo_value(value, env)?);
        Ok(())
    }

    #[napi]
    pub fn data(&self, key: String, env: Env) -> Result<JsUnknown> {
        Ok(match self.builder.data().get(key.as_str()) {
            Some(object) => teo_value_to_js_any(self.builder.app_data(), object, &env)?,
            None => env.get_undefined()?.into_unknown(),
        })
    }
}