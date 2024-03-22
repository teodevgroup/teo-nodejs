pub mod dateonly;
pub mod object_id;
pub mod file;
pub mod range;

pub use dateonly::DateOnly;
pub use object_id::ObjectId;
pub use file::File;
pub use range::Range;

use napi::{Env, Error, JsFunction, JsUnknown, Result};
use teo::prelude::{Value as TeoValue, Value, OptionVariant as TeoOptionVariant};
use super::{interface_enum_variant::teo_interface_enum_variant_to_js_any, model::teo_model_object_to_js_any, pipeline::teo_pipeline_to_js_any, r#struct::teo_struct_object_to_js_any};

#[napi(js_name = "OptionVariant")]
pub struct OptionVariant {
    pub(crate) value: TeoOptionVariant
}

pub fn teo_value_to_js_any(value: &TeoValue, env: &Env) -> Result<JsUnknown> {
    Ok(match value {
        Value::Null => env.get_null()?.into_unknown(),
        Value::Bool(bool) => env.get_boolean(*bool)?.into_unknown(),
        Value::Int(i32) => env.create_int32(*i32)?.into_unknown(),
        Value::Int64(i64) => env.create_int64(*i64)?.into_unknown(),
        Value::Float32(f32) => env.create_double(*f32 as f64)?.into_unknown(),
        Value::Float(f64) => env.create_double(*f64)?.into_unknown(),
        Value::Decimal(decimal) => {
            let global = env.get_global()?;
            let require: JsFunction = global.get_named_property("require")?;
            let decimal_js: JsFunction = unsafe { require.call(None, &[env.create_string("decimal.js")?.into_unknown()])?.cast() };
            let decimal_string = decimal.normalized().to_string();
            decimal_js.call(None, &[env.create_string(&decimal_string)?])?
        },
        Value::ObjectId(object_id) => {
            let instance = ObjectId { value: object_id.clone() }.into_instance(*env)?;
            instance.as_object(*env).into_unknown()
        }
        Value::String(string) => env.create_string(string)?.into_unknown(),
        Value::Date(date) => {
            let instance = DateOnly { value: date.clone() }.into_instance(*env)?;
            instance.as_object(*env).into_unknown()
        }
        Value::DateTime(datetime) => env.create_date(datetime.timestamp_millis() as f64)?.into_unknown(),
        Value::Array(array) => {
            let mut js_array = env.create_array_with_length(array.len())?;
            for (i, value) in array.iter().enumerate() {
                let v = teo_value_to_js_any(value, env)?;
                js_array.set_element(i as u32, &v)?;
            }
            js_array.into_unknown()
        }
        Value::Dictionary(dictionary) => {
            let mut js_object = env.create_object()?;
            for (k, value) in dictionary.iter() {
                let v = teo_value_to_js_any(value, env)?;
                js_object.set_named_property(k, &v)?;
            }
            js_object.into_unknown()
        }
        Value::Range(range) => {
            let instance = Range { value: range.clone() }.into_instance(*env)?;
            instance.as_object(*env).into_unknown()
        }
        Value::Tuple(tuple) => {
            let mut js_array = env.create_array_with_length(tuple.len())?;
            for (i, value) in tuple.iter().enumerate() {
                let v = teo_value_to_js_any(value, env)?;
                js_array.set_element(i as u32, &v)?;
            }
            js_array.into_unknown()
        }
        Value::OptionVariant(option_variant) => {
            let instance = OptionVariant { value: option_variant.clone() }.into_instance(*env)?;
            instance.as_object(*env).into_unknown()
        }
        Value::Regex(regex) => {
            let global = env.get_global()?;
            let reg_exp_constructor: JsFunction = global.get_named_property("RegExp")?;
            let js_regex_str = env.create_string(regex.as_str())?;
            let js_regex = reg_exp_constructor.new_instance(&[js_regex_str])?;
            js_regex.into_unknown()
        }
        Value::File(file) => {
            let instance = File::from(file);
            instance.into_instance(*env)?.as_object(*env).into_unknown()
        }
        Value::ModelObject(model_object) => teo_model_object_to_js_any(model_object, env)?,
        Value::StructObject(struct_object) => teo_struct_object_to_js_any(struct_object, env)?,
        Value::Pipeline(pipeline) => teo_pipeline_to_js_any(pipeline, env)?,
        Value::InterfaceEnumVariant(interface_enum_variant) => teo_interface_enum_variant_to_js_any(interface_enum_variant, env)?,
        _ => Err(Error::new(napi::Status::GenericFailure, "cannot convert Teo value to Python value"))?,
    })
}
