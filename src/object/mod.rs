mod value;
mod model;
mod r#struct;
mod pipeline;
mod interface_enum_variant;
mod array;
mod unused;
mod promise;

use std::collections::HashMap;
use std::str::FromStr;
use bigdecimal::BigDecimal;
use napi::{Env, JsDate, JsString};
use teo::prelude::Value as TeoValue;
use chrono::{NaiveDateTime, NaiveTime, DateTime, Utc};
use napi::{JsUnknown, JsFunction, Result, ValueType};
use napi::bindgen_prelude::{FromNapiValue, Promise};
use napi::sys::{napi_env, napi_value};
use teo::prelude::object::{Object as TeoObject, ObjectInner};
use crate::object::array::teo_array_to_js_any;
use crate::object::interface_enum_variant::teo_interface_enum_variant_to_js_any;
use crate::object::model::teo_model_object_to_js_any;
use crate::object::pipeline::teo_pipeline_to_js_any;
use crate::object::r#struct::teo_struct_object_to_js_any;
use crate::object::value::teo_value_to_js_any;

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

}

pub fn js_unknown_to_teo_value(unknown: JsUnknown, env: Env) -> TeoValue {
    let value_type = unknown.get_type().unwrap();
    match value_type {
        ValueType::Null => TeoValue::Null,
        ValueType::Undefined => TeoValue::Null,
        ValueType::Boolean => TeoValue::Bool(unknown.coerce_to_bool().unwrap().get_value().unwrap()),
        ValueType::Number => {
            let js_number = unknown.coerce_to_number().unwrap();
            if let Ok(n) = js_number.get_int32() {
                TeoValue::I32(n)
            } else if let Ok(n) = js_number.get_int64() {
                TeoValue::I64(n)
            } else if let Ok(f) = js_number.get_double() {
                TeoValue::F64(f)
            } else {
                unreachable!()
            }
        }
        ValueType::String => {
            let js_string = unknown.coerce_to_string().unwrap();
            TeoValue::String(js_string.into_utf8().unwrap().as_str().unwrap().to_owned())
        }
        ValueType::Object => {
            if unknown.is_array().unwrap() {
                let object = unknown.coerce_to_object().unwrap();
                let len = object.get_array_length().unwrap();
                let mut result = vec![];
                for n in 0..len {
                    let item: JsUnknown = object.get_element(n).unwrap();
                    result.push(js_unknown_to_teo_value(item, env));
                }
                TeoValue::Vec(result)
            } else if unknown.is_date().unwrap() {
                let js_date = JsDate::try_from(unknown).unwrap();
                let milliseconds_since_epoch_utc = js_date.value_of().unwrap();
                let milliseconds_since_epoch_utc = milliseconds_since_epoch_utc as i64;
                let timestamp_seconds = milliseconds_since_epoch_utc / 1_000;
                let naive = NaiveDateTime::from_timestamp_opt(
                    timestamp_seconds,
                    (milliseconds_since_epoch_utc % 1_000 * 1_000_000) as u32,
                ).unwrap();
                TeoValue::DateTime(DateTime::<Utc>::from_utc(naive, Utc))
            } else {
                let object = unknown.coerce_to_object().unwrap();
                // test for decimal
                let global = env.get_global().unwrap();
                let require: JsFunction = global.get_named_property("require").unwrap();
                let decimal_js: JsFunction = unsafe { require.call(None, &[env.create_string("decimal.js").unwrap()]).unwrap().cast() };
                if object.instanceof(decimal_js).unwrap() {
                    let js_string = object.coerce_to_string().unwrap();
                    let js_string_utf8 = js_string.into_utf8().unwrap();
                    let s = js_string_utf8.as_str().unwrap();
                    return TeoValue::Decimal(BigDecimal::from_str(s).unwrap());
                } else if object.has_named_property("__teo_object__").unwrap() {
                    let teo_object: &mut TeoObject = env.unwrap(&object).unwrap();
                    return TeoValue::Object(teo_object.clone());
                } else {
                    let mut map = HashMap::new();
                    let names = object.get_property_names().unwrap();
                    let len = names.get_array_length().unwrap();
                    for i in 0..len {
                        let name: JsString = names.get_element(i).unwrap();
                        let v: JsUnknown = object.get_property(name).unwrap();
                        map.insert(name.into_utf8().unwrap().as_str().unwrap().to_owned(), js_unknown_to_teo_value(v, env));
                    }
                    return TeoValue::HashMap(map);
                }
            }
        }
        ValueType::Unknown => {
            panic!("Unhandled Node.js unknown type.")
        }
        _ => {
            panic!("Unhandled Node.js type.")
        }
    }
}
