use time::Duration;
use napi::Result;
use super::expiration::Expiration;

#[napi(js_name = "Cookie")]
pub struct Cookie {
    pub(crate) inner: teo::prelude::request::Cookie<'static>,
}

#[napi]
impl Cookie {

    #[napi(constructor)]
    pub fn new(name: String, value: String) -> Self {
        Self {
            inner: teo::prelude::request::Cookie::new(name, value),
        }
    }

    #[napi]
    pub fn name(&self) -> &str {
        self.inner.name()
    }

    #[napi]
    pub fn value(&self) -> &str {
        self.inner.value()
    }

    #[napi]
    pub fn expires(&self) -> Option<Expiration> {
        self.inner.expires().map(|e| Expiration { inner: e })
    }

    #[napi]
    pub fn max_age(&self) -> Option<f64> {
        self.inner.max_age().map(|ma| ma.as_seconds_f64())
    }

    #[napi]
    pub fn domain(&self) -> Option<&str> {
        self.inner.domain()
    }

    #[napi]
    pub fn path(&self) -> Option<&str> {
        self.inner.path()
    }

    #[napi]
    pub fn secure(&self) -> Option<bool> {
        self.inner.secure()
    }

    #[napi]
    pub fn http_only(&self) -> Option<bool> {
        self.inner.http_only()
    }

    #[napi(ts_args_type = "sameSite: \"strict\" | \"lax\" | \"none\"")]
    pub fn set_same_site(&mut self, same_site: String) -> Result<()> {
        Ok(self.inner.set_same_site(match same_site.as_str() {
            "strict" => teo::prelude::request::SameSite::Strict,
            "lax" => teo::prelude::request::SameSite::Lax,
            "none" => teo::prelude::request::SameSite::None,
            _ => Err(teo_result::Error::internal_server_error_message("invalid same site"))?
        }))
    }

    #[napi(ts_return_type = "\"strict\" | \"lax\" | \"none\"")]
    pub fn same_site(&self) -> Option<&'static str> {
        self.inner.same_site().map(|s| {
            match s {
                teo::prelude::request::SameSite::Strict => "strict",
                teo::prelude::request::SameSite::Lax => "lax",
                teo::prelude::request::SameSite::None => "none",
            }
        })
    }

    #[napi]
    pub fn to_string(&self) -> String {
        self.inner.to_string()
    }

    #[napi]
    pub fn set_max_age(&mut self, max_age: f64) {
        self.inner.set_max_age(Duration::seconds_f64(max_age))
    }

    #[napi]
    pub fn set_expires(&mut self, expires: &Expiration) {
        self.inner.set_expires(expires.inner)
    }

    #[napi]
    pub fn set_domain(&mut self, domain: String) {
        self.inner.set_domain(domain)
    }

    #[napi]
    pub fn set_path(&mut self, path: String) {
        self.inner.set_path(path)
    }

    #[napi]
    pub fn set_secure(&mut self, secure: bool) {
        self.inner.set_secure(secure)
    }

    #[napi]
    pub fn set_http_only(&mut self, http_only: bool) {
        self.inner.set_http_only(http_only)
    }

    #[napi]
    pub fn set_name(&mut self, name: String) {
        self.inner.set_name(name)
    }

    #[napi]
    pub fn set_value(&mut self, value: String) {
        self.inner.set_value(value)
    }

    #[napi]
    pub fn make_removal(&mut self) {
        self.inner.make_removal()
    }

    #[napi]
    pub fn make_permanent(&mut self) {
        self.inner.make_permanent()
    }

    #[napi]
    pub fn from_string(string: String) -> Result<Self> {
        let result = teo::prelude::request::Cookie::parse(string);
        match result {
            Ok(cookie) => Ok(Self { inner: cookie }),
            Err(_) => Err(teo_result::Error::internal_server_error_message("invalid cookie string"))?,
        }
    }
}
