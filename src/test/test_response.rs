use napi::{bindgen_prelude::Buffer, Env, JsFunction, JsGlobal, JsObject, JsUnknown, Result};

#[napi]
pub struct TestResponse {
    teo_test_response: teo::server::test_response::TestResponse,
}

#[napi]
impl TestResponse {
    pub(super) fn new(teo_test_response: teo::server::test_response::TestResponse) -> Self {
        Self {
            teo_test_response,
        }
    }

    #[napi]
    pub fn status(&self) -> u16 {
        self.teo_test_response.status().as_u16()
    }

    #[napi]
    pub fn version(&self) -> String {
        format!("{:?}", self.teo_test_response.version())
    }

    #[napi]
    pub fn body(&self) -> Buffer {
        Buffer::from(Vec::<u8>::from(self.teo_test_response.body().clone()))
    }

    #[napi]
    pub fn body_as_string(&self) -> String {
        self.teo_test_response.body_as_string()
    }

    #[napi(ts_return_type = "any")]
    pub fn body_as_json(&self, env: Env) -> Result<JsUnknown> {
        let string = self.teo_test_response.body_as_string();
        let js_string = env.create_string(&string)?;
        let global: JsGlobal = env.get_global()?;
        let json: JsObject = global.get_named_property("JSON")?;
        let parse: JsFunction = json.get_named_property("parse")?;
        let json_result: JsUnknown = parse.call(None, &[js_string])?;
        Ok(json_result)
    }

    #[napi]
    pub fn contains_header(&self, name: String) -> bool {
        self.teo_test_response.headers().contains_key(name.as_str())
    }

    #[napi]
    pub fn header_value(&self, name: String) -> Result<Option<&str>> {
        let header_value = self.teo_test_response.headers().get(name.as_str());
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
        let header_values = self.teo_test_response.headers().get_all(name.as_str());
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
        let header_map = self.teo_test_response.headers();
        let mut result = vec![];
        header_map.keys().for_each(|k| {
            result.push(k.as_str());
        });
        result
    }

    #[napi]
    pub fn headers_length(&self) -> i64 {
        self.teo_test_response.headers().len() as i64
    }
}