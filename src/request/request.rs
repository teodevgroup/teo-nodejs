use teo::prelude::Request as TeoRequest;
use napi::{Env, JsObject, JsUnknown, Result};

use crate::{dynamic::JSClassLookupMap, object::{js_any_to_teo_value, value::teo_value_to_js_any}};

use super::{Cookie, HandlerMatch};

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
    pub fn version(&self) -> String {
        format!("{:?}", self.teo_request.version())
    }

    #[napi]
    pub fn method(&self) -> &str {
        self.teo_request.method().as_str()
    }

    #[napi]
    pub fn uri(&self) -> String {
        self.teo_request.uri_string()
    }

    #[napi]
    pub fn scheme(&self) -> Option<&str> {
        self.teo_request.scheme_str()
    }

    #[napi]
    pub fn host(&self) -> Option<&str> {
        self.teo_request.host()
    }

    #[napi]
    pub fn path(&self) -> &str {
        self.teo_request.path()
    }

    #[napi]
    pub fn query(&self) -> Option<&str> {
        self.teo_request.query()
    }

    #[napi(js_name = "contentType")]
    pub fn content_type(&self) -> Result<Option<&str>> {
        Ok(self.teo_request.content_type()?)
    }

    #[napi]
    pub fn contains_header(&self, name: String) -> bool {
        self.teo_request.headers().contains_key(name.as_str())
    }

    #[napi]
    pub fn header_value(&self, name: String) -> Result<Option<&str>> {
        let header_value = self.teo_request.headers().get(name.as_str());
        match header_value {
            None => Ok(None),
            Some(header_value) => {
                let header_value = header_value.to_str().map_err(|_| {
                    teo_result::Error::internal_server_error_message(format!("cannot read request header value: {}", name))
                })?;
                Ok(Some(header_value))
            }
        }
    }

    #[napi]
    pub fn header_values(&self, name: String) -> Result<Vec<&str>> {
        let header_values = self.teo_request.headers().get_all(name.as_str());
        let mut result = Vec::new();
        for header_value in header_values {
            let header_value = header_value.to_str().map_err(|_| {
                teo_result::Error::internal_server_error_message(format!("cannot read request header value: {}", name))
            })?;
            result.push(header_value);
        }
        Ok(result)
    }

    #[napi(js_name = "headerKeys", ts_return_type = "string[]")]
    pub fn header_keys(&self) -> Vec<&str> {
        let header_map = self.teo_request.headers();
        let mut result = vec![];
        header_map.keys().for_each(|k| {
            result.push(k.as_str());
        });
        result
    }

    pub fn headers_length(&self) -> usize {
        self.teo_request.headers().len()
    }

    #[napi]
    pub fn cookie(&self, name: String) -> Result<Option<Cookie>> {
        Ok(self.teo_request.cookies()?.get(&name).map(|c| Cookie { inner: c.clone() }))
    }

    #[napi]
    pub fn cookies(&self) -> Result<Vec<Cookie>> {
        Ok(self.teo_request.cookies()?.iter().map(|c| Cookie { inner: c.clone() }).collect())
    }

    #[napi]
    pub fn handler_match(&self) -> Result<HandlerMatch> {
        Ok(HandlerMatch::new(self.teo_request.handler_match()?.clone()))
    }

    #[napi(ts_return_type = "{[key: string]: string}")]
    pub fn captures(&self, env: Env) -> Result<JsObject> {
        let captures_map = self.teo_request.captures()?;
        let mut js_object = env.create_object()?;
        for (k, value) in captures_map.iter() {
            js_object.set_named_property(k, value)?;
        }
        Ok(js_object)
    }

    #[napi(ts_return_type = "any")]
    pub fn body_object(&self, env: Env) -> Result<JsUnknown> {
        teo_value_to_js_any(self.teo_request.transaction_ctx().connection_ctx().namespace().app_data(), self.teo_request.body_value()?, &env)
    }

    #[napi]
    pub fn set_body_object(&self, value: JsUnknown, env: Env) -> Result<()> {
        let teo_value = js_any_to_teo_value(value, env)?;
        self.teo_request.set_body_value(teo_value);
        Ok(())
    }

    #[napi(ts_return_type = "any")]
    pub fn teo(&self, env: Env) -> Result<JsUnknown> {
        let map = JSClassLookupMap::from_app_data(self.teo_request.transaction_ctx().connection_ctx().namespace().app_data());
        Ok(map.teo_transaction_ctx_to_js_ctx_object(env, self.teo_request.transaction_ctx(), "")?.into_unknown())
    }

    // TODO: local objects and local data

    // TODO: take incoming as stream? temp file? string?
}
