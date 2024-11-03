use std::str::FromStr;
use hyper::{header::{HeaderName, HeaderValue}, HeaderMap, Method};
use napi::{JsObject, JsString, Result};

#[napi]
pub struct TestRequest {
    method: Method,
    uri: String,
    headers: HeaderMap,
    body: String,
}

#[napi]
impl TestRequest {
    #[napi(constructor, ts_args_type = "props: { method?: string, uri: string, headers?: { [key: string]: string }, body?: string }")]
    pub fn new(props: JsObject) -> Result<Self> {
        let method: Option<String> = props.get_named_property("method")?;
        let method = match method {
            Some(method) => match Method::from_str(&method) {
                Ok(method) => method,
                Err(_) => Err(teo_result::Error::internal_server_error_message("cannot parse HTTP method"))?,
            },
            None => Method::GET,
        };
        let uri: String = props.get_named_property("uri")?;
        let mut headers: HeaderMap = HeaderMap::new();
        let headers_object: JsObject = props.get_named_property("headers")?;
        let names = headers_object.get_property_names()?;
        let len = names.get_array_length()?;
        for i in 0..len {
            let name: JsString = names.get_element(i)?;
            let v: JsString = headers_object.get_property(name)?;
            headers.insert(match HeaderName::try_from(name.into_utf8()?.as_str()?.to_owned()) {
                Ok(value) => value,
                Err(_) => return Err(teo_result::Error::internal_server_error_message("cannot parse header name").into()),
            }, match HeaderValue::from_str(&v.into_utf8()?.as_str()?.to_owned()) {
                Ok(value) => value,
                Err(_) => return Err(teo_result::Error::internal_server_error_message("cannot parse header value").into()),
            });
        }
        let body: Option<String> = props.get_named_property("body")?;
        let body = body.unwrap_or_default();
        Ok(Self {
            method,
            uri,
            headers,
            body,
        })
    }

    #[napi]
    pub fn method(&self) -> &str {
        self.method.as_str()
    }

    #[napi]
    pub fn set_method(&mut self, method: String) -> Result<()> {
        match Method::from_str(&method) {
            Ok(method) => {
                self.method = method;
                Ok(())
            },
            Err(_) => Err(teo_result::Error::internal_server_error_message("cannot parse HTTP method").into()),
        }
    }

    #[napi]
    pub fn uri(&self) -> &str {
        self.uri.as_str()
    }

    #[napi]
    pub fn set_uri(&mut self, uri: String) {
        self.uri = uri;
    }

    #[napi]
    pub fn insert_header(&mut self, key: String, value: String) -> Result<()> {
        self.headers.insert(match HeaderName::try_from(key) {
            Ok(value) => value,
            Err(_) => return Err(teo_result::Error::internal_server_error_message("cannot parse header name").into()),
        }, match HeaderValue::from_str(value.as_str()) {
            Ok(value) => value,
            Err(_) => return Err(teo_result::Error::internal_server_error_message("cannot parse header value").into()),
        });
        Ok(())
    }

    #[napi]
    pub fn append_header(&mut self, key: String, value: String) -> Result<()> {
        self.headers.append(match HeaderName::try_from(key) {
            Ok(value) => value,
            Err(_) => return Err(teo_result::Error::internal_server_error_message("cannot parse header name").into()),
        }, match HeaderValue::from_str(value.as_str()) {
            Ok(value) => value,
            Err(_) => return Err(teo_result::Error::internal_server_error_message("cannot parse header value").into()),
        });
        Ok(())
    }

    #[napi]
    pub fn body(&self) -> &str {
        self.body.as_str()
    }

    #[napi]
    pub fn set_body(&mut self, body: String) {
        self.body = body;
    }
}
