use napi::{Env, Result, JsObject};
use teo::prelude::handler::r#match::HandlerMatch as TeoHandlerMatch;

pub struct HandlerMatch {
    inner: TeoHandlerMatch,
}

/// Handler match.
#[napi]
impl HandlerMatch {

    pub(crate) fn new(inner: TeoHandlerMatch) -> Self {
        Self {
            inner
        }
    }

    #[napi]
    pub fn path(&self) -> &Vec<String> {
        self.inner.path()
    }

    #[napi]
    pub fn handler_name(&self) -> &str {
        self.inner.handler_name()
    }

    #[napi(ts_return_type = "{[key: string]: string} | any")]
    pub fn captures(&self, env: Env) -> Result<JsObject> {
        let captures_map = self.inner.captures();
        let mut js_object = env.create_object()?;
        for (k, value) in captures_map.iter() {
            js_object.set_named_property(k, value)?;
        }
        Ok(js_object)
    }

}
