use napi::{Env, JsUnknown, Ref, Result};
use teo::prelude::request::local_objects::LocalObjects as TeoLocalObjects;

#[napi(js_name = "LocalObjects")]
pub struct LocalObjects {
    pub(crate) teo_local_objects: TeoLocalObjects,
}

#[napi]
impl LocalObjects {

    #[napi]
    pub fn insert(&self, key: String, value: JsUnknown, env: Env) -> Result<()> {
        let reference = env.create_reference(value)?;
        self.teo_local_objects.insert(key, reference);
        Ok(())
    }

    #[napi]
    pub fn get(&self, key: String, env: Env) -> Result<Option<JsUnknown>> {
        let reference: Option<&Ref<()>> = self.teo_local_objects.get(&key);
        match reference {
            Some(reference) => {
                let value: JsUnknown = env.get_reference_value(reference)?;
                Ok(Some(value))
            },
            None => Ok(None),
        }
    }

    #[napi]
    pub fn contains(&self, key: String) -> bool {
        self.teo_local_objects.contains::<Ref<()>>(&key)
    }

    #[napi]
    pub fn remove(&self, key: String) {
        self.teo_local_objects.remove::<Ref<()>>(key.as_str());
    }

    #[napi]
    pub fn clear(&self) {
        self.teo_local_objects.clear();
    }
}