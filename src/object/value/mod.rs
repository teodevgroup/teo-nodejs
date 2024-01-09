pub mod dateonly;
pub mod object_id;

pub use dateonly::DateOnly;

use napi::{Env, JsFunction, JsUnknown, Result};
use teo::prelude::{Value as TeoValue, Value, Range as TeoRange, File as TeoFile, EnumVariant as TeoEnumVariant, OptionVariant as TeoOptionVariant};
use bson::oid::ObjectId as BsonObjectId;

#[napi(js_name = "ObjectId")]
pub struct ObjectId {
    pub(crate) value: BsonObjectId,
}

#[napi(js_name = "Range")]
pub struct Range {
    pub(crate) value: TeoRange
}

#[napi(js_name = "File")]
pub struct File {
    pub(crate) value: TeoFile
}

#[napi(js_name = "EnumVariant")]
pub struct EnumVariant {
    pub(crate) value: TeoEnumVariant
}

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
        Value::DateTime(datetime) => env.create_date(datetime.timestamp() as f64)?.into_unknown(),
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
        Value::EnumVariant(enum_variant) => {
            let instance = EnumVariant { value: enum_variant.clone() }.into_instance(*env)?;
            instance.as_object(*env).into_unknown()
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
            let instance = File { value: file.clone() }.into_instance(*env)?;
            instance.as_object(*env).into_unknown()
        }
    })
}
