use teo::prelude::cookies::{Cookie as OriginalCookie, SameSite};
use time::Duration;
use napi::Result;
use super::expiration::Expiration;

#[napi]
pub struct Cookie {
    original: OriginalCookie
}

impl From<OriginalCookie> for Cookie {
    fn from(original: OriginalCookie) -> Self {
        Self { original }
    }
}

impl Cookie {
    pub(super) fn original(&self) -> &OriginalCookie {
        &self.original
    }
}

#[napi]
impl Cookie {

    #[napi(constructor)]
    pub fn new(args: CookieCreateArgs) -> Result<Self> {
        let original = OriginalCookie::new(args.name, args.value);
        let cookie = Self::from(original);
        if let Some(http_only) = args.http_only {
            cookie.set_http_only(Some(http_only));
        }
        if let Some(secure) = args.secure {
            cookie.set_secure(Some(secure));
        }
        if let Some(same_site) = args.same_site {
            cookie.set_same_site(Some(same_site))?;
        }
        if let Some(partitioned) = args.partitioned {
            cookie.set_partitioned(Some(partitioned));
        }
        if let Some(max_age) = args.max_age {
            cookie.set_max_age(Some(max_age));
        }
        if let Some(path) = args.path {
            cookie.set_path(Some(path));
        }
        if let Some(domain) = args.domain {
            cookie.set_domain(Some(domain));
        }
        if let Some(expires) = args.expires {
            cookie.set_expires(Some(&expires));
        }
        Ok(cookie)
    }

    #[napi(getter)]
    pub fn name(&self) -> String {
        self.original.name()
    }

    #[napi(setter)]
    pub fn set_name(&self, name: String) {
        self.original.set_name(name);
    }

    #[napi(getter)]
    pub fn value(&self) -> String {
        self.original.value()
    }

    #[napi(setter)]
    pub fn set_value(&self, value: String) {
        self.original.set_value(value);
    }

    #[napi(getter)]
    pub fn value_trimmed(&self) -> String {
        self.original.value_trimmed()
    }

    #[napi(getter)]
    pub fn http_only(&self) -> Option<bool> {
        self.original.http_only()
    }

    #[napi(setter, ts_args_type = "httpOnly: boolean | null | undefined")]
    pub fn set_http_only(&self, http_only: Option<bool>) {
        self.original.set_http_only(http_only)
    }

    #[napi(getter)]
    pub fn secure(&self) -> Option<bool> {
        self.original.secure()
    }

    #[napi(setter, ts_args_type = "secure: boolean | null | undefined")]
    pub fn set_secure(&self, secure: Option<bool>) {
        self.original.set_secure(secure)
    }

    #[napi(getter, ts_return_type = "\"strict\" | \"lax\" | \"none\"")]
    pub fn same_site(&self) -> Option<&'static str> {
        self.original.same_site().map(|s| {
            match s {
                SameSite::Strict => "strict",
                SameSite::Lax => "lax",
                SameSite::None => "none",
            }
        })
    }

    #[napi(setter, ts_args_type = "sameSite: \"strict\" | \"lax\" | \"none\"")]
    pub fn set_same_site(&self, same_site: Option<String>) -> Result<()> {
        Ok(self.original.set_same_site(match same_site {
            None => None,
            Some(same_site) => match same_site.as_str() {
                "strict" => Some(SameSite::Strict),
                "lax" => Some(SameSite::Lax),
                "none" => Some(SameSite::None),
                _ => Err(teo_result::Error::internal_server_error_message("invalid same site"))?
            }
        }))
    }

    #[napi(getter)]
    pub fn partitioned(&self) -> Option<bool> {
        self.original.partitioned()
    }

    #[napi(setter, ts_args_type = "partitioned: boolean | null | undefined")]
    pub fn set_partitioned(&self, partitioned: Option<bool>) {
        self.original.set_partitioned(partitioned)
    }

    #[napi(getter)]
    pub fn max_age(&self) -> Option<f64> {
        self.original.max_age().map(|ma| ma.as_seconds_f64())
    }

    #[napi(setter, ts_args_type = "maxAge: number | null | undefined")]
    pub fn set_max_age(&self, max_age: Option<f64>) {
        self.original.set_max_age(max_age.map(|d| Duration::seconds(d as i64)));
    }

    #[napi(getter)]
    pub fn path(&self) -> Option<String> {
        self.original.path()
    }

    #[napi(setter, ts_args_type = "path: string | null | undefined")]
    pub fn set_path(&self, path: Option<String>) {
        self.original.set_path(path)
    }

    #[napi(getter)]
    pub fn domain(&self) -> Option<String> {
        self.original.domain()
    }

    #[napi(setter, ts_args_type = "domain: string | null | undefined")]
    pub fn set_domain(&self, domain: Option<String>) {
        self.original.set_domain(domain)
    }

    #[napi(getter)]
    pub fn expires(&self) -> Option<Expiration> {
        self.original.expires().map(|e| e.into())
    }

    #[napi(setter, ts_args_type = "expires: Expiration | null | undefined")]
    pub fn set_expires(&self, expires: Option<&Expiration>) {
        self.original.set_expires(expires.map(|e| e.clone().original().clone()));
    }

    pub fn make_removal(&self) {
        self.original.make_removal()
    }

    pub fn make_permanent(&self) {
        self.original.make_permanent()
    }

    pub fn encoded(&self) -> String {
        self.original.encoded()
    }
}

#[napi(object)]
pub struct CookieCreateArgs {
    pub name: String,
    pub value: String,
    pub http_only: Option<bool>,
    pub secure: Option<bool>,
    pub same_site: Option<String>,
    pub partitioned: Option<bool>,
    pub max_age: Option<f64>,
    pub path: Option<String>,
    pub domain: Option<String>,
    pub expires: Option<Expiration>,
}