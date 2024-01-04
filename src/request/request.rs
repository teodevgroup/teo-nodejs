
use teo::prelude::Request as TeoRequest;
use napi::Env;

use super::header_map::ReadOnlyHeaderMap;


#[napi(js_name = "Request")]
pub struct Request {
    pub(crate) teo_request: TeoRequest,
}

/// HTTP request.
#[napi]
impl Request {

    pub(crate) fn new(teo_request: TeoRequest) -> Self {
        Self {
            teo_request
        }
    }

    #[napi]
    pub fn method(&self, env: Env) -> &str {
        self.teo_request.method()
    }

    #[napi]
    pub fn path(&self, env: Env) -> &str {
        self.teo_request.path()
    }

    #[napi(js_name = "queryString")]
    pub fn query_string(&self, env: Env) -> &str {
        self.teo_request.query_string()
    }

    #[napi(js_name = "contentType")]
    pub fn content_type(&self, env: Env) -> &str {
        self.teo_request.content_type()
    }

    #[napi]
    pub fn headers(&self, env: Env) -> ReadOnlyHeaderMap {
        ReadOnlyHeaderMap {
            inner: self.teo_request.headers().clone()
        }
    }
}
