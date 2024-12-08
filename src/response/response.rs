use std::path::PathBuf;
use napi::{Result, JsUnknown, Env, bindgen_prelude::{FromNapiValue, FromNapiRef}};
use teo::prelude::response::Response as OriginalResponse;
use crate::{cookies::Cookies, headers::headers::Headers, object::{js_any_to_teo_value, value::teo_value_to_js_any_no_map}};

#[derive(Clone)]
#[napi(js_name = "Response")]
pub struct Response {
    pub(crate) original: OriginalResponse,
}

#[napi]
impl Response {

    #[napi]
    pub fn empty() -> Self {
        Self {
            original: OriginalResponse::empty()
        }
    }

    #[napi]
    pub fn string(content: String, content_type: String) -> Result<Self> {
        Ok(Self {
            original: OriginalResponse::string(content, &content_type.as_str())?
        })
    }

    #[napi(ts_args_type = "value: any", ts_return_type = "Response")]
    pub fn teon(value: JsUnknown, env: Env) -> Result<Self> {
        let teo_value = js_any_to_teo_value(value, env)?;
        let response = OriginalResponse::teon(teo_value);
        Ok(Self {
            original: response
        })
    }

    #[napi]
    pub fn html(content: String) -> Result<Self> {
        Ok(Self::string(content, "text/html".to_owned())?)
    }

    #[napi(ts_args_type = "value: any", ts_return_type = "Response")]
    pub fn data(value: JsUnknown, env: Env) -> Result<Self> {
        let teo_value = js_any_to_teo_value(value, env)?;
        let response = OriginalResponse::data(teo_value);
        Ok(Self {
            original: response
        })
    }
    
    #[napi(ts_args_type = "data: any, meta: any", ts_return_type = "Response")]
    pub fn data_meta(data: JsUnknown, meta: JsUnknown, env: Env) -> Result<Self> {
        let teo_data = js_any_to_teo_value(data, env)?;
        let teo_meta = js_any_to_teo_value(meta, env)?;
        let response = OriginalResponse::data_meta(teo_data, teo_meta);
        Ok(Self {
            original: response
        })
    }
    
    #[napi(js_name = "file")]
    pub fn file(path: String) -> Self {
        let path_buf = PathBuf::from(path);
        Self {
            original: OriginalResponse::file(path_buf)
        }
    }

    #[napi(js_name = "sendFile")]
    pub fn send_file(base: String, path: String) -> Result<Self> {
        Ok(Self {
            original: OriginalResponse::send_file(base, path)?
        })
    }

    #[napi(js_name = "redirect")]
    pub fn redirect(path: String) -> Result<Self> {
        Ok(Self {
            original: OriginalResponse::redirect(path)?
        })
    }

    #[napi(setter)]
    pub fn set_code(&self, code: u16) {
        self.original.set_code(code)
    }

    #[napi(getter)]
    pub fn code(&self) -> u16 {
        self.original.code()
    }

    #[napi(getter)]
    pub fn headers(&self) -> Headers {
        self.original.headers().into()
    }

    #[napi(setter)]
    pub fn set_headers(&self, headers: &Headers) {
        self.original.set_headers(headers.original().clone());
    }

    #[napi(getter)]
    pub fn is_file(&self) -> bool {
        self.original.body().is_file()
    }

    #[napi(getter)]
    pub fn is_text(&self) -> bool {
        self.original.body().is_text()
    }

    #[napi(getter)]
    pub fn is_empty(&self) -> bool {
        self.original.body().is_empty()
    }

    #[napi(getter)]
    pub fn is_teon(&self) -> bool {
        self.original.body().is_teon()
    }

    #[napi]
    pub fn get_text(&self) -> Option<String> {
        self.original.body().as_text().cloned()
    }

    #[napi(ts_return_type = "any | null")]
    pub fn get_teon(&self, env: Env) -> Result<JsUnknown> {
        Ok(match self.original.body().as_teon() {
            None => env.get_undefined()?.into_unknown(),
            Some(value) => teo_value_to_js_any_no_map(value, &env)?
        })
    }

    #[napi]
    pub fn get_file(&self) -> Option<String> {
        match self.original.body().as_file() {
            None => None,
            Some(path_buf) => Some(path_buf.to_str().unwrap().to_string()),
        }
    }

    #[napi(getter)]
    pub fn cookies(&self) -> Cookies {
        self.original.cookies().into()
    }

    #[napi(setter)]
    pub fn set_cookies(&self, cookies: &Cookies) {
        self.original.set_cookies(cookies.original().clone());
    }
}

impl FromNapiValue for Response {
    unsafe fn from_napi_value(env: napi::sys::napi_env, napi_val: napi::sys::napi_value) -> Result<Self> {
        let response: &Response = Response::from_napi_ref(env, napi_val)?;
        Ok(response.clone())
    }
}
