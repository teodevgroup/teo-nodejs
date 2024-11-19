use napi::{bindgen_prelude::{FromNapiValue, Reference, WeakReference}, JsUnknown, Result};
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
    pub fn insert(&self, key: String, value: JsUnknown) -> Result<()> {
        let reference: Reference<JsUnknown> = Reference::from_unknown(value)?;
        //let reference = env.create_reference(value)?;
        self.teo_local_objects.insert(key, reference);
        Ok(())
    }

    #[napi(ts_return_type = "any")]
    pub fn get(&self, key: String) -> Result<Option<WeakReference<JsUnknown>>> {
        let reference: Option<&Reference<JsUnknown>> = self.teo_local_objects.get(&key);
        match reference {
            Some(reference) => {
                Ok(Some(reference.downgrade().clone()))
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