use std::collections::HashMap;
use std::str::FromStr;
use bigdecimal::BigDecimal;
use napi::{Env, JsDate, JsString};
use teo::core::teon::Value as TeoValue;
use chrono::{NaiveDateTime, NaiveTime, DateTime, Utc};
use napi::{JsUnknown, threadsafe_function::ThreadSafeCallContext, JsFunction, Result, ValueType};
use napi::bindgen_prelude::{FromNapiValue, Promise};
use napi::sys::{napi_env, napi_value};
use teo::core::object::Object as TeoObject;

pub fn teo_value_to_js_unknown<T>(value: &TeoValue, ctx: &ThreadSafeCallContext<T>) -> JsUnknown {
    match value {
        TeoValue::String(s) => ctx.env.create_string(s).unwrap().into_unknown(),
        TeoValue::Bool(b) => ctx.env.get_boolean(*b).unwrap().into_unknown(),
        TeoValue::I64(i) => ctx.env.create_int64(*i).unwrap().into_unknown(),
        TeoValue::I32(i) => ctx.env.create_int32(*i).unwrap().into_unknown(),
        TeoValue::F32(f) => ctx.env.create_double(*f as f64).unwrap().into_unknown(),
        TeoValue::F64(f) => ctx.env.create_double(*f).unwrap().into_unknown(),
        TeoValue::Date(d) => ctx.env.create_date(NaiveDateTime::new(*d, NaiveTime::default()).timestamp() as f64).unwrap().into_unknown(),
        TeoValue::DateTime(d) => ctx.env.create_date(d.timestamp() as f64).unwrap().into_unknown(),
        TeoValue::Decimal(d) => {
            let global = ctx.env.get_global().unwrap();
            let require: JsFunction = global.get_named_property("require").unwrap();
            let decimal_js: JsFunction = unsafe { require.call(None, &[ctx.env.create_string("decimal.js").unwrap().into_unknown()]).unwrap().cast() };
            let decimal_string = d.normalized().to_string();
            decimal_js.call(None, &[ctx.env.create_string(&decimal_string).unwrap()]).unwrap()
        },
        TeoValue::Vec(v) => {
            let mut js_array = ctx.env.create_array_with_length(v.len()).unwrap();
            for (i, value) in v.iter().enumerate() {
                let v = teo_value_to_js_unknown(value, ctx);
                let _ = js_array.set_element(i as u32, &v);
            }
            js_array.into_unknown()
        }
        TeoValue::HashMap(m) => { // how to reduce duplication here?
            let mut js_object = ctx.env.create_object().unwrap();
            for (k, value) in m.iter() {
                let v = teo_value_to_js_unknown(value, ctx);
                let _ = js_object.set_named_property(k, &v);
            }
            js_object.into_unknown()
        }
        TeoValue::IndexMap(m) => {
            let mut js_object = ctx.env.create_object().unwrap();
            for (k, value) in m.iter() {
                let v = teo_value_to_js_unknown(value, ctx);
                let _ = js_object.set_named_property(k, &v);
            }
            js_object.into_unknown()
        }
        TeoValue::BTreeMap(m) => {
            let mut js_object = ctx.env.create_object().unwrap();
            for (k, value) in m.iter() {
                let v = teo_value_to_js_unknown(value, ctx);
                let _ = js_object.set_named_property(k, &v);
            }
            js_object.into_unknown()
        }
        _ => panic!("Unhandled type")
    }
}

pub enum WrappedTeoValue {
    Promise(Promise<WrappedTeoValue>),
    TeoValue(TeoValue),
}

unsafe impl Send for WrappedTeoValue {}
unsafe impl Sync for WrappedTeoValue {}

impl WrappedTeoValue {
    pub async fn to_teo_value(self) -> TeoValue {
        match self {
            WrappedTeoValue::Promise(promise) => match promise.await {
                Ok(p) => match p {
                    WrappedTeoValue::Promise(_) => TeoValue::String("Nested promise".to_string()),
                    WrappedTeoValue::TeoValue(v) => v,
                },
                Err(e) => TeoValue::String(e.reason.clone()),
            },
            WrappedTeoValue::TeoValue(v) => v,
        }
    }
}

impl FromNapiValue for WrappedTeoValue {
    unsafe fn from_napi_value(raw_env: napi_env, napi_val: napi_value) -> Result<Self> {
        let env = Env::from_raw(raw_env);
        let unknown = JsUnknown::from_napi_value(raw_env, napi_val).unwrap();
        if unknown.is_promise().unwrap() {
            let promise: Promise<WrappedTeoValue> = Promise::from_napi_value(raw_env, napi_val).unwrap();
            Ok(WrappedTeoValue::Promise(promise))
        } else {
            Ok(WrappedTeoValue::TeoValue(js_unknown_to_teo_value(unknown, env)))
        }
    }
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
