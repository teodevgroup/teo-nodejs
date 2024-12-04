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
        self.original.get(&key).map(|cookie| Cookie {
            original: cookie.clone(),
        })
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
}