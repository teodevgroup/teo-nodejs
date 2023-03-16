use teo::core::teon::Value as TeoValue;
use chrono::{NaiveDateTime, NaiveTime};
use napi::{JsUnknown, threadsafe_function::ThreadSafeCallContext, JsFunction, Result, ValueType};
use napi::bindgen_prelude::FromNapiValue;
use napi::sys::{napi_env, napi_value};

pub fn teo_value_to_js_unknown(value: &TeoValue, ctx: &ThreadSafeCallContext<TeoValue>) -> JsUnknown {
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
            let decimal_js: JsFunction = unsafe { require.call(None, &[ctx.env.create_string("decimal.js").unwrap()]).unwrap().cast() };
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

pub struct WrappedTeoValue { value: TeoValue }

impl WrappedTeoValue {
    pub fn to_teo_value(self) -> TeoValue {
        self.value
    }
}

impl FromNapiValue for WrappedTeoValue {
    unsafe fn from_napi_value(env: napi_env, napi_val: napi_value) -> napi::Result<Self> {
        let unknown = JsUnknown::from_napi_value(env, napi_val).unwrap();
        Ok(WrappedTeoValue { value: js_unknown_to_teo_value(unknown) })
    }
}

fn js_unknown_to_teo_value(unknown: JsUnknown) -> TeoValue {
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
            panic!("Unhandled Node.js object type.")
        }
        ValueType::Unknown => {
            panic!("Unhandled Node.js unknown type.")
        }
        _ => {
            panic!("Unhandled Node.js type.")
        }
    }
}
