pub(crate) mod value;
mod model;
mod r#struct;
mod pipeline;
mod interface_enum_variant;
mod array;
mod unused;
pub(crate) mod promise;
pub(crate) mod arguments;
pub(crate) mod promise_or_ignore;
pub(crate) mod unknown;

use indexmap::IndexMap;
use std::str::FromStr;
use bigdecimal::BigDecimal;
use napi::{Env, Error, JsDate, JsString, Status};
use teo::prelude::{Value as TeoValue, Value};
use chrono::{NaiveDateTime, NaiveTime, DateTime, Utc};
use napi::{JsUnknown, JsFunction, Result, ValueType};
use teo::prelude::object::{Object as TeoObject, ObjectInner};
use regex::Regex;
use crate::object::array::teo_array_to_js_any;
use crate::object::interface_enum_variant::{InterfaceEnumVariant, teo_interface_enum_variant_to_js_any};
use crate::object::model::teo_model_object_to_js_any;
use crate::object::pipeline::{Pipeline, teo_pipeline_to_js_any};
use crate::object::r#struct::teo_struct_object_to_js_any;
use crate::object::value::{DateOnly, EnumVariant, File, ObjectId, OptionVariant, Range, teo_value_to_js_any};

pub fn teo_object_to_js_any(object: &TeoObject, env: &Env) -> Result<JsUnknown> {
    match object.inner.as_ref() {
        ObjectInner::Teon(value) => teo_value_to_js_any(value, env),
        ObjectInner::ModelObject(model_object) => teo_model_object_to_js_any(model_object, env),
        ObjectInner::StructObject(struct_object) => teo_struct_object_to_js_any(struct_object, env),
        ObjectInner::Pipeline(pipeline) => teo_pipeline_to_js_any(pipeline, env),
        ObjectInner::InterfaceEnumVariant(interface_enum_variant) => teo_interface_enum_variant_to_js_any(interface_enum_variant, env),
        ObjectInner::Array(array) => teo_array_to_js_any(array, env),
    }
}

pub fn js_any_to_teo_object(any: JsUnknown, env: Env) -> Result<TeoObject> {
    Ok(match any.get_type()? {
        ValueType::Undefined => TeoObject::from(Value::Null),
        ValueType::Null => TeoObject::from(Value::Null),
        ValueType::Boolean => TeoObject::from(TeoValue::Bool(any.coerce_to_bool()?.get_value().unwrap())),
        ValueType::Number => TeoObject::from({
            let js_number = any.coerce_to_number()?;
            if let Ok(n) = js_number.get_int32() {
                TeoValue::Int(n)
            } else if let Ok(n) = js_number.get_int64() {
                TeoValue::Int64(n)
            } else if let Ok(f) = js_number.get_double() {
                TeoValue::Float(f)
            } else {
                Err(Error::new(Status::Unknown, "cannot convert number value to teon number"))?
            }
        }),
        ValueType::String => TeoObject::from({
            let js_string = any.coerce_to_string()?;
            TeoValue::String(js_string.into_utf8()?.as_str()?.to_owned())
        }),
        ValueType::Symbol => Err(Error::new(Status::Unknown, "cannot convert symbol to teon value"))?,
        ValueType::Object => {
            if any.is_array()? {
                let object = any.coerce_to_object()?;
                let len = object.get_array_length()?;
                let mut result: Vec<TeoObject> = vec![];
                for n in 0..len {
                    let item: JsUnknown = object.get_element(n).unwrap();
                    result.push(js_any_to_teo_object(item, env)?);
                }
                if result.is_empty() {
                    TeoObject::from(TeoValue::Array(vec![]))
                } else if result.first().unwrap().is_teon() {
                    let teon_result: Vec<Value> = result.iter().map(|r| r.as_teon().unwrap().clone()).collect();
                    TeoObject::from(teon_result)
                } else {
                    TeoObject::from(result)
                }
            } else if any.is_date()? {
                let js_date = JsDate::try_from(any)?;
                let milliseconds_since_epoch_utc = js_date.value_of()?;
                let milliseconds_since_epoch_utc = milliseconds_since_epoch_utc as i64;
                let timestamp_seconds = milliseconds_since_epoch_utc / 1_000;
                let naive = NaiveDateTime::from_timestamp_opt(
                    timestamp_seconds,
                    (milliseconds_since_epoch_utc % 1_000 * 1_000_000) as u32,
                ).unwrap();
                TeoObject::from(TeoValue::DateTime(DateTime::<Utc>::from_utc(naive, Utc)))
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
                    return Ok(TeoObject::from(TeoValue::Decimal(BigDecimal::from_str(s).unwrap())));
                }
                // test for object id
                if ObjectId::instance_of(env, &object)? {
                    let object_id: &mut ObjectId = env.unwrap(&object)?;
                    return Ok(TeoObject::from(TeoValue::ObjectId(object_id.value.clone())));
                }
                // test for date only
                if DateOnly::instance_of(env, &object)? {
                    let date_only: &mut DateOnly = env.unwrap(&object)?;
                    return Ok(TeoObject::from(TeoValue::Date(date_only.value.clone())));
                }
                // test for date time
                if object.is_date()? {
                    let number: JsFunction = global.get_named_property("Number")?;
                    let milliseconds = number.new_instance(&[object])?.coerce_to_number()?.get_int64()?;
                    let ts_secs = milliseconds / 1000;
                    let ts_ns = (milliseconds % 1000) * 1_000_000 ;
                    let datetime = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(ts_secs, ts_ns as u32), Utc);
                    return Ok(TeoObject::from(TeoValue::DateTime(datetime)));
                }
                // test for regex
                let reg_exp: JsFunction = global.get_named_property("RegExp")?;
                if object.instanceof(reg_exp)? {
                    let source: JsFunction = object.get_named_property("source")?;
                    let source_unknown: JsUnknown = source.call_without_args(Some(&object))?;
                    let source_string: JsString = source_unknown.coerce_to_string()?;
                    let rust_string = source_string.into_utf8()?.as_str()?.to_owned();
                    let regex = Regex::new(&rust_string).unwrap();
                    return Ok(TeoObject::from(TeoValue::Regex(regex)));
                }
                // test for range
                if Range::instance_of(env, &object)? {
                    let range: &mut Range = env.unwrap(&object)?;
                    return Ok(TeoObject::from(TeoValue::Range(range.value.clone())));
                }
                // test for file
                if File::instance_of(env, &object)? {
                    let file: &mut File = env.unwrap(&object)?;
                    return Ok(TeoObject::from(TeoValue::File(file.value.clone())));
                }
                // test for enum variant
                if EnumVariant::instance_of(env, &object)? {
                    let enum_variant: &mut EnumVariant = env.unwrap(&object)?;
                    return Ok(TeoObject::from(TeoValue::EnumVariant(enum_variant.value.clone())));
                }
                // test for option variant
                if OptionVariant::instance_of(env, &object)? {
                    let enum_variant: &mut OptionVariant = env.unwrap(&object)?;
                    return Ok(TeoObject::from(TeoValue::OptionVariant(enum_variant.value.clone())));
                }
                // test for interface enum variant
                if InterfaceEnumVariant::instance_of(env, &object)? {
                    let enum_variant: &mut InterfaceEnumVariant = env.unwrap(&object)?;
                    return Ok(TeoObject::from(enum_variant.value.clone()));
                }
                // test for pipeline
                if Pipeline::instance_of(env, &object)? {
                    let pipeline: &mut Pipeline = env.unwrap(&object)?;
                    return Ok(TeoObject::from(pipeline.value.clone()));
                }
                // // test for model object
                // if object.has_named_property("__teo_object__")? {
                //     let model_object: &mut ModelObject = env.unwrap(&object)?;
                //     return Ok(TeoObject::from(model_object.clone()));
                // }
                // otherwise, treat as default dictionary
                let mut map = IndexMap::new();
                let names = object.get_property_names()?;
                let len = names.get_array_length()?;
                for i in 0..len {
                    let name: JsString = names.get_element(i)?;
                    let v: JsUnknown = object.get_property(name)?;
                    map.insert(name.into_utf8()?.as_str()?.to_owned(), js_any_to_teo_object(v, env)?.as_teon().unwrap().clone());
                }
                return Ok(TeoObject::from(TeoValue::Dictionary(map)));
            }
        },
        ValueType::Function => Err(Error::new(Status::Unknown, "cannot convert function to teon value"))?,
        #[cfg(feature = "napi6")]
        ValueType::BigInt => Err(Error::new(Status::Unknown, "cannot convert big int to teon value"))?,
        ValueType::External => Err(Error::new(Status::Unknown, "cannot convert external to teon value"))?,
        ValueType::Unknown => Err(Error::new(Status::Unknown, "cannot convert unknown to teon value"))?,
    })
}
