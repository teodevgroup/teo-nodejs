use hyper::{HeaderMap, StatusCode, Version};
use napi::Result;

#[napi]
pub struct TestResponse {
    headers: HeaderMap,
    status: StatusCode,
    version: Version,
    body: String,
}

#[napi]
impl TestResponse {
    pub fn status(&self) -> u16 {
        self.status.as_u16()
    }

    pub fn version(&self) -> String {
        format!("{:?}", self.version)
    }

    pub fn body(&self) -> &str {
        self.body.as_str()
    }

    #[napi]
    pub fn contains_header(&self, name: String) -> bool {
        self.headers.contains_key(name.as_str())
    }

    #[napi]
    pub fn header_value(&self, name: String) -> Result<Option<&str>> {
        let header_value = self.headers.get(name.as_str());
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
        let header_values = self.headers.get_all(name.as_str());
        let mut result = Vec::new();
        for header_value in header_values {
            let header_value = header_value.to_str().map_err(|_| {
                teo_result::Error::internal_server_error_message(format!("cannot read request header value: {}", name))
            })?;
            result.push(header_value);
        }
        Ok(result)
    }

    #[napi(js_name = "header_keys", ts_return_type = "string[]")]
    pub fn header_keys(&self) -> Vec<&str> {
        let header_map = &self.headers;
        let mut result = vec![];
        header_map.keys().for_each(|k| {
            result.push(k.as_str());
        });
        result
    }

    pub fn headers_length(&self) -> usize {
        self.headers.len()
    }
}