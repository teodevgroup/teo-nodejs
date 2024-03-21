use napi::{Env, JsObject, Result};
use teo::prelude::Arguments;

use super::value::teo_value_to_js_any;

pub(crate) fn teo_args_to_js_args(args: &Arguments, env: &Env) -> Result<JsObject> {
    let mut js_object = env.create_object()?;
    for (k, v) in args.iter() {
        let v = teo_value_to_js_any(v, env)?;
        js_object.set_named_property(k, &v)?;
    }
    Ok(js_object)
}