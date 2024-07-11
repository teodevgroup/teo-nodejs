use teo::prelude::Request as TeoRequest;
use napi::{Env, JsObject, Result};

use super::Cookie;

#[napi(js_name = "Request")]
pub struct Request {
    pub(crate) teo_request: TeoRequest,
}

/// HTTP request.
#[napi]
impl Request {

    #[napi]
    pub fn method(&self) -> &str {
        self.teo_request.method()
    }

    #[napi]
    pub fn path(&self) -> &str {
        self.teo_request.path()
    }

    #[napi(js_name = "queryString")]
    pub fn query_string(&self) -> &str {
        self.teo_request.query_string()
    }

    #[napi(js_name = "contentType")]
    pub fn content_type(&self) -> &str {
        self.teo_request.content_type()
    }

    #[napi]
    pub fn header(&self, name: String) -> Option<&str> {
        let header_value = self.teo_request.headers().get(name.as_str());
        header_value.map(|hv| hv.to_str().unwrap())
    }

    #[napi(js_name = "headers", ts_return_type = "{[key: string]: string}")]
    pub fn headers(&self, env: Env) -> Result<JsObject> {
        let header_map = self.teo_request.headers();
        let mut object = env.create_object()?;
        for (k, v) in header_map.iter() {
            object.set_named_property(k.as_str(), v.to_str().unwrap().to_owned())?;
        }
        Ok(object)
    }

    #[napi]
    pub fn cookie(&self, name: String) -> Option<Cookie> {
        self.teo_request.cookie(&name).map(|c| Cookie { inner: c })
    }

    #[napi]
    pub fn cookies(&self) -> Result<Vec<Cookie>> {
        Ok(self.teo_request.cookies()?.iter().map(|c| Cookie { inner: c.clone() }).collect())
    }
}
