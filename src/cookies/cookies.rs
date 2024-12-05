use napi::{Env, JsFunction, JsUnknown, Result};
use teo::prelude::cookies::Cookies as OriginalCookies;
use super::cookie::Cookie;

#[napi]
pub struct Cookies {
    original: OriginalCookies
}

impl From<OriginalCookies> for Cookies {
    fn from(original: OriginalCookies) -> Self {
        Cookies { original }
    }
}

#[napi]
impl Cookies {

    #[napi(constructor)]
    pub fn new(cookies: Option<Vec<&Cookie>>) -> Self {
        let original = OriginalCookies::new();
        if let Some(cookies) = cookies {
            original.set_entries(cookies.into_iter().map(|c| c.original().clone()).collect());
        }
        Cookies { original }
    }

    #[napi]
    pub fn get(&self, key: String) -> Option<Cookie> {
        self.original.get(&key).map(|cookie| Cookie::from(cookie))
    }

    #[napi]
    pub fn has(&self, key: String) -> bool {
        self.original.has(&key)
    }

    #[napi]
    pub fn push(&self, cookie: &Cookie) {
        self.original.push(cookie.original().clone());
    }

    #[napi]
    pub fn clear(&self) {
        self.original.clear()
    }

    #[napi(getter)]
    pub fn length(&self) -> i64 {
        self.original.len() as i64
    }

    #[napi(js_name="map", ts_args_type="callback: (cookie: Cookie) => T", ts_return_type="T[]")]
    pub fn map(&self, callback: JsFunction, env: Env) -> Result<Vec<JsUnknown>> {
        let entries = self.original.entries();
        let mut result = Vec::with_capacity(entries.len());
        for entry in entries {
            let cookie = Cookie::from(entry);
            let cookie_instance = cookie.into_instance(env)?;
            let js_result = callback.call(None, &[cookie_instance])?;
            result.push(js_result);
        }
        Ok(result)
    }
}