mod header_map;

use crate::{object::js_any_to_teo_object, result::IntoNodeJSResult};

use self::header_map::ReadWriteHeaderMap;
use napi::{Result, JsUnknown, Env};
use teo::prelude::response::Response as TeoResponse;

#[napi(js_name = "Response")]
pub struct Response {
    pub(crate) teo_response: TeoResponse,
}

#[napi]
impl Response {

    #[napi]
    pub fn empty() -> Self {
        Self {
            teo_response: TeoResponse::empty()
        }
    }

    #[napi]
    pub fn string(content: String, content_type: String) -> Self {
        Self {
            teo_response: TeoResponse::string(content, &content_type.as_str())
        }
    }

    #[napi]
    pub fn json(value: JsUnknown, env: Env) -> Result<Self> {
        let teo_value = js_any_to_teo_object(value, env)?.as_teon().unwrap().clone();
        let response = TeoResponse::json(teo_value).into_nodejs_result()?;
        Ok(Self {
            teo_response: response
        })
    }

    #[napi]
    pub fn html(content: String) -> Self {
        Self::string(content, "text/html".to_owned())
    }

    #[napi]
    pub fn data(value: JsUnknown, env: Env) -> Result<Self> {
        let teo_value = js_any_to_teo_object(value, env)?.as_teon().unwrap().clone();
        let response = TeoResponse::data(teo_value).into_nodejs_result()?;
        Ok(Self {
            teo_response: response
        })
    }
    
    #[napi(js_name = "dataMeta")]
    pub fn data_meta(data: JsUnknown, meta: JsUnknown, env: Env) -> Result<Self> {
        let teo_data = js_any_to_teo_object(data, env)?.as_teon().unwrap().clone();
        let teo_meta = js_any_to_teo_object(meta, env)?.as_teon().unwrap().clone();
        let response = TeoResponse::data_meta(teo_data, teo_meta).into_nodejs_result()?;
        Ok(Self {
            teo_response: response
        })
    }

    // error
    
    // file

    #[napi(js_name = "redirect")]
    pub fn redirect(path: String) -> Self {
        Self {
            teo_response: TeoResponse::redirect(path)
        }
    }

    #[napi(js_name = "setCode")]
    pub fn set_code(&self, code: u16) {
        self.teo_response.set_code(code)
    }

    #[napi(js_name = "code")]
    pub fn code(&self) -> u16 {
        self.teo_response.code()
    }

    #[napi(js_name = "headers")]
    pub fn headers(&self) -> ReadWriteHeaderMap {
        ReadWriteHeaderMap {
            inner: self.teo_response.headers()
        }
    }

    // body
}
