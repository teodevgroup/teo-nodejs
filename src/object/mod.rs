pub(crate) mod value;
mod r#struct;
mod pipeline;
mod interface_enum_variant;
mod unused;
pub(crate) mod promise;
pub(crate) mod arguments;
pub(crate) mod promise_or_ignore;
pub(crate) mod unknown;

use indexmap::IndexMap;
use napi::bindgen_prelude::FromNapiRef;
use std::str::FromStr;
use bigdecimal::BigDecimal;
use napi::{Env, Error, JsDate, JsString, Status, NapiRaw};
use teo::prelude::{Value as TeoValue, Value};
use chrono::{NaiveDateTime, DateTime, Utc};
use napi::{JsUnknown, JsFunction, Result, ValueType};
use teo::prelude::File as TeoFile;
use regex::Regex;
use crate::object::interface_enum_variant::InterfaceEnumVariant;
use crate::object::value::{DateOnly, File, ObjectId, OptionVariant, Range};
use crate::pipeline::pipeline::Pipeline;

pub fn js_any_to_teo_value(any: JsUnknown, env: Env) -> Result<Value> {
    Ok(match any.get_type()? {
        ValueType::Undefined => Value::Null,
        ValueType::Null => Value::Null,
        ValueType::Boolean => TeoValue::Bool(any.coerce_to_bool()?.get_value().unwrap()),
        ValueType::Number => {
            let js_number = any.coerce_to_number()?;
            if let Ok(n) = js_number.get_double() {
                TeoValue::Float(n)
            } else if let Ok(n) = js_number.get_int32() {
                TeoValue::Int(n)
            } else if let Ok(n) = js_number.get_int64() {
                TeoValue::Int64(n)
            } else {
                Err(Error::new(Status::Unknown, "cannot convert number value to teon number"))?
            }
        },
        ValueType::String => {
            let js_string = any.coerce_to_string()?;
            TeoValue::String(js_string.into_utf8()?.as_str()?.to_owned())
        },
        ValueType::Symbol => Err(Error::new(Status::Unknown, "cannot convert symbol to teon value"))?,
        ValueType::Object => {
            if any.is_array()? {
                let object = any.coerce_to_object()?;
                let len = object.get_array_length()?;
                let mut result: Vec<Value> = vec![];
                for n in 0..len {
                    let item: JsUnknown = object.get_element(n).unwrap();
                    result.push(js_any_to_teo_value(item, env)?);
                }
                Value::Array(result)
            } else if any.is_date()? {
                let js_date = JsDate::try_from(any)?;
                let milliseconds_since_epoch_utc = js_date.value_of()?;
                let milliseconds_since_epoch_utc = milliseconds_since_epoch_utc as i64;
                let rust_date = DateTime::<Utc>::from_timestamp_millis(milliseconds_since_epoch_utc).unwrap();
                TeoValue::DateTime(rust_date)
            } else {
                let object = any.coerce_to_object()?;
                // test for decimal
                let global = env.get_global()?;
                let require: JsFunction = global.get_named_property("require")?;
                let decimal_js: JsFunction = unsafe { require.call(None, &[env.create_string("decimal.js")?])?.cast() };
                if object.instanceof(decimal_js)? {
                    let js_string = object.coerce_to_string()?;
                    let js_string_utf8 = js_string.into_utf8()?;
                    let s = js_string_utf8.as_str()?;
                    return Ok(TeoValue::Decimal(BigDecimal::from_str(s).unwrap()));
                }
                // test for object id
                if ObjectId::instance_of(env, &object)? {
                    let object_id: &ObjectId = unsafe { ObjectId::from_napi_ref(env.raw(), object.raw())? };
                    return Ok(TeoValue::ObjectId(object_id.original.clone()));
                }
                // test for date only
                if DateOnly::instance_of(env, &object)? {
                    let date_only: &DateOnly = unsafe { DateOnly::from_napi_ref(env.raw(), object.raw())? };
                    return Ok(TeoValue::Date(date_only.original.clone()));
                }
                // test for date time
                if object.is_date()? {
                    let number: JsFunction = global.get_named_property("Number")?;
                    let milliseconds = number.new_instance(&[object])?.coerce_to_number()?.get_int64()?;
                    let ts_secs = milliseconds / 1000;
                    let ts_ns = (milliseconds % 1000) * 1_000_000;
                    let datetime = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(ts_secs, ts_ns as u32), Utc);
                    return Ok(TeoValue::DateTime(datetime));
                }
                // test for regex
                let reg_exp: JsFunction = global.get_named_property("RegExp")?;
                if object.instanceof(reg_exp)? {
                    let source: JsFunction = object.get_named_property("source")?;
                    let source_unknown: JsUnknown = source.call_without_args(Some(&object))?;
                    let source_string: JsString = source_unknown.coerce_to_string()?;
                    let rust_string = source_string.into_utf8()?.as_str()?.to_owned();
                    let regex = Regex::new(&rust_string).unwrap();
                    return Ok(TeoValue::Regex(regex));
                }
                // test for range
                if Range::instance_of(env, &object)? {
                    let range: &Range = unsafe { Range::from_napi_ref(env.raw(), object.raw())? };
                    return Ok(TeoValue::Range(range.value.clone()));
                }
                // test for file
                if File::instance_of(env, &object)? {
                    let file: &File = unsafe { File::from_napi_ref(env.raw(), object.raw())? };
                    let teo_file = TeoFile::from(file);
                    return Ok(TeoValue::File(teo_file));
                }
                // test for option variant
                if OptionVariant::instance_of(env, &object)? {
                    let enum_variant: &OptionVariant = unsafe { OptionVariant::from_napi_ref(env.raw(), object.raw())? };
                    return Ok(TeoValue::OptionVariant(enum_variant.original.clone()));
                }
                // test for interface enum variant
                if InterfaceEnumVariant::instance_of(env, &object)? {
                    let enum_variant: &InterfaceEnumVariant = unsafe { InterfaceEnumVariant::from_napi_ref(env.raw(), object.raw())? };
                    return Ok(TeoValue::InterfaceEnumVariant(enum_variant.original.clone()));
                }
                // test for pipeline
                if Pipeline::instance_of(env, &object)? {
                    let pipeline: &Pipeline = unsafe { Pipeline::from_napi_ref(env.raw(), object.raw())? };
                    return Ok(TeoValue::Pipeline(pipeline.original.clone()));
                }
                // test for model object
                if object.has_named_property("__teo_object__")? {
                    let model_object: &mut teo::prelude::model::Object = env.unwrap(&object)?;
                    return Ok(Value::from(model_object.clone()));
                }
                // otherwise, treat as default dictionary
                let mut map = IndexMap::new();
                let names = object.get_property_names()?;
                let len = names.get_array_length()?;
                for i in 0..len {
                    let name: JsString = names.get_element(i)?;
                    let v: JsUnknown = object.get_property(name)?;
                    map.insert(name.into_utf8()?.as_str()?.to_owned(), js_any_to_teo_value(v, env)?);
                }
                return Ok(TeoValue::Dictionary(map));
            }
        },
        ValueType::Function => Err(Error::new(Status::Unknown, "cannot convert function to teon value"))?,
        #[cfg(feature = "napi6")]
        ValueType::BigInt => Err(Error::new(Status::Unknown, "cannot convert big int to teon value"))?,
        ValueType::External => Err(Error::new(Status::Unknown, "cannot convert external to teon value"))?,
        ValueType::Unknown => Err(Error::new(Status::Unknown, "cannot convert unknown to teon value"))?,
    })
}
