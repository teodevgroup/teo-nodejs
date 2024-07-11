use time::{self, OffsetDateTime};
use napi::{Env, JsDate, Result};

#[napi(js_name = "Expiration")]
pub struct Expiration {
    pub(crate) inner: teo::prelude::request::Expiration
}

#[napi]
impl Expiration {

    #[napi]
    pub fn create_session() -> Self {
        Expiration { inner: teo::prelude::request::Expiration::Session }
    }

    #[napi]
    pub fn create_datetime(datetime: JsDate) -> Result<Self> {
        let js_number = datetime.coerce_to_number()?;
        let js_value = js_number.get_uint32()?;
        Ok(Expiration { inner: teo::prelude::request::Expiration::DateTime(OffsetDateTime::from_unix_timestamp_nanos(js_value as i128 * 1000000).unwrap()) })
    }

    #[napi]
    pub fn is_session(&self) -> bool {
        self.inner.is_session()
    }

    #[napi]
    pub fn is_datetime(&self) -> bool {
        self.inner.is_datetime()
    }

    #[napi]
    pub fn datetime(&self, env: Env) -> Option<JsDate> {
        self.inner.datetime().map(|dt| env.create_date((dt.unix_timestamp_nanos() as f64) / 1000000.0 as f64).unwrap())
    }
}