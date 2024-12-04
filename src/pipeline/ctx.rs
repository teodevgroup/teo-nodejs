use napi::{Env, JsObject, JsUnknown, Result};
use teo::prelude::pipeline;
use key_path::Item;
use crate::{dynamic::JSClassLookupMap, object::value::{teo_model_object_to_js_any, teo_value_to_js_any}, request::Request};

#[napi]
pub struct PipelineCtx {
    original: pipeline::Ctx,
    pub(crate) map: &'static JSClassLookupMap,
}

impl From<pipeline::Ctx> for PipelineCtx {
    fn from(original: pipeline::Ctx) -> Self {
        let map = JSClassLookupMap::from_app_data(original.object().namespace().app_data());
        Self { 
            original,
            map,
        }
    }
}

#[napi]
impl PipelineCtx {

    #[napi(getter)]
    pub fn value(&self, env: Env) -> Result<JsUnknown> {
        Ok(teo_value_to_js_any(self.map, self.original.value(), &env)?)
    }

    #[napi(getter)]
    pub fn object(&self, env: Env) -> Result<JsUnknown> {
        Ok(teo_model_object_to_js_any(self.map, self.original.object(), &env)?)
    }

    #[napi(getter)]
    pub fn path(&self, env: Env) -> Result<JsObject> {
        let keypath = self.original.path();
        let mut js_array = env.create_array_with_length(keypath.len())?;
        for (index, item) in keypath.iter().enumerate() {
            match item {
                Item::Index(n) => js_array.set_element(index as u32, env.create_uint32(*n as u32)?)?,
                Item::Key(k) => js_array.set_element(index as u32, env.create_string(k)?)?,
            }
        }
        Ok(js_array)
    }

    #[napi(getter)]
    pub fn teo(&self, env: Env) -> Result<JsObject> {
        self.map.teo_transaction_ctx_to_js_ctx_object(env, self.original.transaction_ctx(), "")
    }

    #[napi(getter)]
    pub fn request(&self) -> Option<Request> {
        self.original.request().map(|r| Request::from(r))
    }
}