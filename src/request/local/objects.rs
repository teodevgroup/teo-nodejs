use napi::{Env, JsUnknown, Ref, Result};
use teo::prelude::request::local_objects::LocalObjects as OriginalLocalObjects;

#[napi]
pub struct LocalObjects {
    pub(crate) original: OriginalLocalObjects,
}

impl From<OriginalLocalObjects> for LocalObjects {
    fn from(original: OriginalLocalObjects) -> Self {
        Self { original }
    }
}

#[napi]
impl LocalObjects {

    #[napi(ts_args_type = "key: string, value: any")]
    pub fn set(&self, key: String, value: JsUnknown, env: Env) -> Result<()> {
        let reference = env.create_reference(value)?;
        self.original.insert(key, reference);
        Ok(())
    }

    #[napi(ts_return_type = "any")]
    pub fn get(&self, key: String, env: Env) -> Result<Option<JsUnknown>> {
        let reference: Option<&mut Ref<()>> = self.original.get_mut(&key);
        match reference {
            Some(reference) => {
                let any: JsUnknown = env.get_reference_value(reference)?;
                Ok(Some(any))
            },
            None => Ok(None),
        }
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