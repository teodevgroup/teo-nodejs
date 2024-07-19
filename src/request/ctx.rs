use teo::prelude::{request::Ctx as TeoRequestCtx, App as TeoApp};
use napi::{Env, JsObject, JsUnknown, Result};
use crate::{dynamic::JSClassLookupMap, object::value::teo_value_to_js_any};

use super::{Request, HandlerMatch};

#[napi(js_name = "RequestCtx")]
pub struct RequestCtx {
    teo_inner: TeoRequestCtx,
    app: &'static TeoApp,
}

/// HTTP request.
#[napi]
impl RequestCtx {

    pub(crate) fn new(teo_inner: TeoRequestCtx, app: &'static TeoApp) -> Self {
        Self {
            teo_inner,
            app,
        }
    }

    #[napi]
    pub fn request(&self) -> Request {
        Request {
            teo_request: self.teo_inner.request().clone()
        }
    }

    #[napi(ts_return_type = "any")]
    pub fn body(&self, env: Env) -> Result<JsUnknown> {
        teo_value_to_js_any(self.app, self.teo_inner.body(), &env)
    }

    #[napi(ts_return_type = "any")]
    pub fn teo(&self, env: Env) -> Result<JsUnknown> {
        let map = JSClassLookupMap::from_app(self.app);
        Ok(map.teo_transaction_ctx_to_js_ctx_object(env, self.teo_inner.transaction_ctx(), "")?.into_unknown())
    }

    #[napi]
    pub fn handler_match(&'static self) -> HandlerMatch {
        HandlerMatch::new(self.teo_inner.handler_match())
    }

    #[napi(ts_return_type = "any")]
    pub fn path_arguments(&'static self, env: Env) -> Result<JsObject> {
        self.handler_match().captures(env)
    }
}
