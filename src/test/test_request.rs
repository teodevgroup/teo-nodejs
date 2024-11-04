use std::str::FromStr;
use hyper::{header::{HeaderName, HeaderValue}, HeaderMap, Method};
use napi::{bindgen_prelude::Buffer, Env, JsBuffer, JsFunction, JsObject, JsString, JsUnknown, Result, ValueType};
use http_body_util::Full;
use bytes::Bytes;

#[napi]
pub struct TestRequest {
    method: Method,
    uri: String,
    headers: HeaderMap,
    body: Bytes,
}

#[napi]
impl TestRequest {
    #[napi(constructor, ts_args_type = "props: { method?: string, uri: string, headers?: { [key: string]: string }, body?: any }")]
    pub fn new(props: JsObject, env: Env) -> Result<Self> {
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
        let headers_object: Option<JsObject> = props.get_named_property("headers")?;
        if let Some(headers_object) = headers_object {
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
        }
        let body: Option<JsUnknown> = props.get_named_property("body")?;
        let body = match body {
            Some(body) => {
                let value_type = body.get_type()?;
                match value_type {
                    ValueType::String => {
                        let js_string = body.coerce_to_string()?;
                        Bytes::copy_from_slice(js_string.into_utf8()?.as_slice())
                    },
                    _ => {
                        let global = env.get_global()?;
                        // detect buffer
                        let buffer_class: JsFunction = global.get_named_property("Buffer")?;
                        if body.instanceof(buffer_class)? {
                            let buf: JsBuffer = props.get_named_property("body")?;
                            Bytes::copy_from_slice(AsRef::<[u8]>::as_ref(&buf.into_value()?))
                        } else {
                            // anything else, convert to JSON string
                            let json: JsObject = global.get_named_property("JSON")?;
                            let stringify: JsFunction = json.get_named_property("stringify")?;
                            let json_string: JsString = stringify.call(None, &[body])?.coerce_to_string()?;
                            Bytes::copy_from_slice(json_string.into_utf8()?.as_slice())
                        }
                    }
                }
            },
            None => Bytes::new(),
        };
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
    pub fn body(&self) -> Buffer {
        Buffer::from(Vec::<u8>::from(self.body.clone()))
    }

    #[napi]
    pub fn set_body(&mut self, body: Buffer) {
        let body_vec: Vec<u8> = Vec::<u8>::from(body);
        self.body = Bytes::copy_from_slice(&body_vec);
    }

    pub(crate) fn to_hyper_request(&self) -> hyper::Request<Full<Bytes>> {
        let mut request = hyper::Request::builder()
            .method(self.method.clone())
            .uri(self.uri.clone());
        for (key, value) in self.headers.iter() {
            request = request.header(key.clone(), value.clone());
        }
        request.body(Full::new(self.body.clone())).unwrap()
    }
}
