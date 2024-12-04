use napi::{Env, Result, JsObject};
use teo::prelude::handler::r#match::HandlerMatch as OriginalHandlerMatch;

#[napi]
pub struct HandlerMatch {
    original: OriginalHandlerMatch,
}

impl From<OriginalHandlerMatch> for HandlerMatch {
    fn from(original: OriginalHandlerMatch) -> Self {
        Self { original }
    }
}

/// Handler match.
#[napi]
impl HandlerMatch {

    #[napi]
    pub fn path(&self) -> &Vec<String> {
        self.original.path()
    }

    #[napi]
    pub fn handler_name(&self) -> &str {
        self.original.handler_name()
    }

    #[napi(ts_return_type = "{[key: string]: string} | any")]
    pub fn captures(&self, env: Env) -> Result<JsObject> {
        let captures_map = self.original.captures();
        let mut js_object = env.create_object()?;
        for (k, value) in captures_map.iter() {
            js_object.set_named_property(k, value)?;
        }
        Ok(js_object)
    }
}
