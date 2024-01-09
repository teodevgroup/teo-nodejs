use napi::{Env, JsUnknown, Result};
use teo::prelude::model;

use crate::dynamic::js_model_object_from_teo_model_object;

pub fn teo_model_object_to_js_any(model_object: &model::Object, env: &Env) -> Result<JsUnknown> {
    let js_object = js_model_object_from_teo_model_object(*env, model_object.clone())?;
    Ok(js_object.into_unknown())
}