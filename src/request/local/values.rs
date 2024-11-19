use napi::{Env, JsUnknown, Result};
use teo::prelude::{request::local_values::LocalValues as TeoLocalValues, Value};

#[napi(js_name = "LocalValues")]
pub struct LocalValues {
    pub(crate) teo_local_values: TeoLocalValues,
}

#[napi]
impl LocalValues {

    #[napi(ts_args_type = "key: string, value: any")]
    pub fn insert(&self, key: String, value: JsUnknown, env: Env) -> Result<()> {
        let value = crate::object::js_any_to_teo_value(value, env)?;
        self.teo_local_values.insert(key, value);
        Ok(())
    }

    #[napi(ts_return_type= "any")]
    pub fn get(&self, key: String, env: Env) -> Result<JsUnknown> {
        let value: &Value = self.teo_local_values.get(&key)?;
        Ok(crate::object::value::teo_value_to_js_any_no_app_data(value, &env)?)
    }

    #[napi]
    pub fn contains(&self, key: String) -> bool {
        self.teo_local_values.contains(&key)
    }

    #[napi]
    pub fn remove(&self, key: String) {
        self.teo_local_values.remove(key.as_str());
    }

    #[napi]
    pub fn clear(&self) {
        self.teo_local_values.clear();
    }
}