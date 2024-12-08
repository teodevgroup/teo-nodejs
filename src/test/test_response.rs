use napi::{bindgen_prelude::Buffer, Env, JsFunction, JsGlobal, JsObject, JsUnknown, Result};
use teo::server::test_response::TestResponse as OriginalTestResponse;
use crate::{cookies::Cookies, headers::headers::Headers};

#[napi]
pub struct TestResponse {
    original: OriginalTestResponse,
}

impl From<OriginalTestResponse> for TestResponse {
    fn from(original: OriginalTestResponse) -> Self {
        Self { original }
    }
}

#[napi]
impl TestResponse {

    #[napi(getter)]
    pub fn status(&self) -> u16 {
        self.original.status().as_u16()
    }

    #[napi(getter)]
    pub fn version(&self) -> String {
        format!("{:?}", self.original.version())
    }

    #[napi]
    pub fn body(&self) -> Buffer {
        Buffer::from(Vec::<u8>::from(self.original.body().clone()))
    }

    #[napi]
    pub fn body_as_string(&self) -> String {
        self.original.body_as_string()
    }

    #[napi(ts_return_type = "any")]
    pub fn body_as_json(&self, env: Env) -> Result<JsUnknown> {
        let string = self.original.body_as_string();
        let js_string = env.create_string(&string)?;
        let global: JsGlobal = env.get_global()?;
        let json: JsObject = global.get_named_property("JSON")?;
        let parse: JsFunction = json.get_named_property("parse")?;
        let json_result: JsUnknown = parse.call(None, &[js_string])?;
        Ok(json_result)
    }

    #[napi(getter)]
    pub fn headers(&self) -> Headers {
        Headers::from(self.original.headers().clone())
    }

    #[napi(getter)]
    pub fn cookies(&self) -> Cookies {
        Cookies::from(self.original.cookies().clone())
    }
}