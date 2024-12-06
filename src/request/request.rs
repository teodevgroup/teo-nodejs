use teo::prelude::{request::{Version, Method, Request as OriginalRequest}, Error};
use napi::{Env, JsObject, JsUnknown, Result};
use crate::{cookies::Cookies, dynamic::{DynamicClasses, QueryDynamicClasses}, headers::headers::Headers, object::{js_any_to_teo_value, value::teo_value_to_js_any}};
use super::{local::{objects::LocalObjects, values::LocalValues}, HandlerMatch};

#[napi(js_name = "Request")]
pub struct Request {
    pub(crate) original: OriginalRequest,
}

impl From<OriginalRequest> for Request {
    fn from(original: OriginalRequest) -> Self {
        Self { original }
    }
}

/// Represents an incoming HTTP request.
#[napi]
impl Request {

    #[napi(getter)]
    pub fn version(&self) -> String {
        format!("{:?}", self.original.version())
    }

    #[napi(setter)]
    pub fn set_version(&self, value: String) -> Result<()> {
        match value.as_str() {
            "HTTP/0.9" => { self.original.set_version(Version::HTTP_09); },
            "HTTP/1.0" => { self.original.set_version(Version::HTTP_10); },
            "HTTP/1.1" => { self.original.set_version(Version::HTTP_11); },
            "HTTP/2.0" => { self.original.set_version(Version::HTTP_2); },
            "HTTP/3.0" => { self.original.set_version(Version::HTTP_3); },
            _ => { Err(Error::new(format!("Invalid version: {}", value)))? },
        }
        Ok(())
    }

    #[napi(getter)]
    pub fn method(&self) -> &str {
        self.original.method().as_str()
    }

    #[napi(setter)]
    pub fn set_method(&self, value: String) -> Result<()> {
        let method = Method::from_bytes(value.as_bytes()).unwrap();
        self.original.set_method(method);
        Ok(())
    }

    #[napi(getter)]
    pub fn uri(&self) -> String {
        self.original.uri_string()
    }

    #[napi(setter)]
    pub fn set_uri(&self, value: String) -> Result<()> {
        self.original.set_uri_string(value.as_str())?;
        Ok(())
    }

    #[napi(getter)]
    pub fn scheme(&self) -> Option<&str> {
        self.original.scheme_str()
    }

    #[napi(getter)]
    pub fn host(&self) -> Option<&str> {
        self.original.host()
    }

    #[napi(getter)]
    pub fn path(&self) -> &str {
        self.original.path()
    }

    #[napi(getter)]
    pub fn query(&self) -> Option<&str> {
        self.original.query()
    }

    #[napi(getter)]
    pub fn content_type(&self) -> Result<Option<String>> {
        Ok(self.original.content_type()?)
    }

    #[napi(getter)]
    pub fn headers(&self) -> Headers {
        self.original.headers().into()
    }

    #[napi(getter)]
    pub fn cookies(&self) -> Result<Cookies> {
        Ok(self.original.cookies()?.clone().into())
    }

    #[napi(getter)]
    pub fn handler_match(&self) -> Result<HandlerMatch> {
        Ok(self.original.handler_match()?.clone().into())
    }

    #[napi(getter, ts_return_type = "{[key: string]: string} | any")]
    pub fn captures(&self, env: Env) -> Result<JsObject> {
        self.handler_match()?.captures(env)
    }

    #[napi(getter, ts_return_type = "any")]
    pub fn body_object(&self, env: Env) -> Result<JsUnknown> {
        let dynamic_classes = DynamicClasses::retrieve(self.original.transaction_ctx().connection_ctx().namespace().app_data())?;
        teo_value_to_js_any(&dynamic_classes, self.original.body_value()?, &env)
    }

    #[napi(setter)]
    pub fn set_body_object(&self, value: JsUnknown, env: Env) -> Result<()> {
        let teo_value = js_any_to_teo_value(value, env)?;
        self.original.set_body_value(teo_value);
        Ok(())
    }

    #[napi(getter, ts_return_type = "any")]
    pub fn teo(&self, env: Env) -> Result<JsUnknown> {
        let dynamic_classes = DynamicClasses::retrieve(self.original.transaction_ctx().connection_ctx().namespace().app_data())?;
        Ok(dynamic_classes.teo_transaction_ctx_to_js_ctx_object(env, self.original.transaction_ctx(), "")?.into_unknown())
    }

    #[napi(getter)]
    pub fn local_values(&self) -> LocalValues {
        LocalValues {
            original: self.original.local_values().clone(),
        }
    }

    #[napi(getter)]
    pub fn local_objects(&self) -> LocalObjects {
        LocalObjects {
            original: self.original.local_objects().clone(),
        }
    }

    // TODO: take incoming as stream? temp file? string?
}
