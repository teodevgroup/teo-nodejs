use napi::{Env, JsUnknown, Result};
use teo::prelude::r#struct;

pub fn teo_struct_object_to_js_any(_struct_object: &r#struct::Object, _env: &Env) -> Result<JsUnknown> {
    unreachable!()
}