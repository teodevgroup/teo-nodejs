use napi::{Env, JsUnknown, Result};
use teo::prelude::{request::local_values::LocalValues as OriginalLocalValues, Value};

#[napi(js_name = "LocalValues")]
#[derive(Clone)]
pub struct LocalValues {
    pub(crate) original: OriginalLocalValues,
}

impl From<OriginalLocalValues> for LocalValues {
    fn from(original: OriginalLocalValues) -> Self {
        Self { original }
    }
}

#[napi]
impl LocalValues {

    #[napi(ts_args_type = "key: string, value: any")]
    pub fn set(&self, key: String, value: JsUnknown, env: Env) -> Result<()> {
        let value = crate::object::js_any_to_teo_value(value, env)?;
        self.original.insert(key, value);
        Ok(())
    }

    #[napi(ts_return_type= "any")]
    pub fn get(&self, key: String, env: Env) -> Result<JsUnknown> {
        let value: &Value = self.original.get(&key)?;
        Ok(crate::object::value::teo_value_to_js_any_no_map(value, &env)?)
    }

    #[napi]
    pub fn has(&self, key: String) -> bool {
        self.original.contains(&key)
    }

    #[napi]
    pub fn remove(&self, key: String) {
        self.original.remove(key.as_str());
    }

    #[napi]
    pub fn clear(&self) {
        self.original.clear();
    }
}