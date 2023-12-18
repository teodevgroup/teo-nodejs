use napi::{Env, JsUnknown, Result};
use teo::prelude::object::Object;
use crate::object::teo_object_to_js_any;

pub fn teo_array_to_js_any(array: &Vec<Object>, env: &Env) -> Result<JsUnknown> {
    let mut js_array = env.create_array_with_length(array.len())?;
    for (i, value) in array.iter().enumerate() {
        let v = teo_object_to_js_any(value, env)?;
        js_array.set_element(i as u32, &v)?;
    }
    Ok(js_array.into_unknown())
}