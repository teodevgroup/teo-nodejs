use napi::{Result, JsUnknown, Env};
use teo::prelude::Model as TeoModel;

use crate::object::{teo_object_to_js_any, js_any_to_teo_object};

#[napi(js_name = "Model")]
pub struct Model {
    pub(crate) teo_model: &'static mut TeoModel,
}

#[napi]
impl Model {

    #[napi]
    pub fn set_data(&mut self, key: String, value: JsUnknown, env: Env) -> Result<()> {
        self.teo_model.data.insert(key, js_any_to_teo_object(value, env)?);
        Ok(())
    }

    #[napi]
    pub fn data(&mut self, key: String, env: Env) -> Result<JsUnknown> {
        Ok(match self.teo_model.data.get(key.as_str()) {
            Some(object) => teo_object_to_js_any(object, &env)?,
            None => env.get_undefined()?.into_unknown(),
        })
    }
}