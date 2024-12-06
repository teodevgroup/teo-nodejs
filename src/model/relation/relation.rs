use napi::{Result, JsUnknown, Env};
use teo::prelude::model;
use crate::{dynamic::DynamicClasses, object::{js_any_to_teo_value, value::teo_value_to_js_any}};

#[napi(js_name = "Relation")]
pub struct Relation {
    pub(crate) builder: model::relation::Builder,
}

#[napi]
impl Relation {

    #[napi]
    pub fn set_data(&self, key: String, value: JsUnknown, env: Env) -> Result<()> {
        self.builder.data().insert(key, js_any_to_teo_value(value, env)?);
        Ok(())
    }

    #[napi]
    pub fn data(&self, key: String, env: Env) -> Result<JsUnknown> {
        let dynamic_classes = DynamicClasses::retrieve(self.builder.app_data())?;
        Ok(match self.builder.data().get(key.as_str()) {
            Some(object) => teo_value_to_js_any(&dynamic_classes, object, &env)?,
            None => env.get_undefined()?.into_unknown(),
        })
    }
}