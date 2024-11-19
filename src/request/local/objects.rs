use napi::{bindgen_prelude::{FromNapiValue, Reference, WeakReference}, Env, JsUnknown, Ref, Result};
use teo::prelude::request::local_objects::LocalObjects as TeoLocalObjects;

#[napi(js_name = "LocalObjects")]
pub struct LocalObjects {
    pub(crate) teo_local_objects: TeoLocalObjects,
}

impl LocalObjects {
    pub(crate) fn new(teo_local_objects: TeoLocalObjects) -> Self {
        Self { teo_local_objects }
    }
}

#[napi]
impl LocalObjects {

    #[napi(ts_args_type = "key: string, value: any")]
    pub fn insert(&self, key: String, value: JsUnknown, env: Env) -> Result<()> {
        let reference = env.create_reference(value)?;
        self.teo_local_objects.insert(key, reference);
        Ok(())
    }

    #[napi(ts_return_type = "any")]
    pub fn get(&self, key: String, env: Env) -> Result<Option<JsUnknown>> {
        let reference: Option<&mut Ref<()>> = self.teo_local_objects.get_mut(&key);
        match reference {
            Some(reference) => {
                let any: JsUnknown = env.get_reference_value(reference)?;
                Ok(Some(any))
            },
            None => Ok(None),
        }
    }

    #[napi]
    pub fn contains(&self, key: String) -> bool {
        self.teo_local_objects.contains(&key)
    }

    #[napi]
    pub fn remove(&self, key: String) {
        self.teo_local_objects.remove(key.as_str());
    }

    #[napi]
    pub fn clear(&self) {
        self.teo_local_objects.clear();
    }
}