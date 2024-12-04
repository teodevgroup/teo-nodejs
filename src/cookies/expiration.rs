use napi::bindgen_prelude::{FromNapiRef, FromNapiValue};
use time::{self, OffsetDateTime};
use chrono::{DateTime, Utc};
use teo::prelude::cookies::Expiration as OriginalExpiration;

#[napi]
#[derive(Clone)]
pub struct Expiration {
    original: OriginalExpiration
}

impl Expiration {
    pub(super) fn original(&self) -> &OriginalExpiration {
        &self.original
    }
}

impl From<OriginalExpiration> for Expiration {
    fn from(original: OriginalExpiration) -> Self {
        Self { original }
    }
}

#[napi]
impl Expiration {

    #[napi(factory)]
    pub fn session_expiration() -> Self {
        Expiration { original: OriginalExpiration::Session }
    }

    #[napi(factory)]
    pub fn datetime_expiration(datetime: DateTime<Utc>) -> Self {
        let timestamp = datetime.timestamp_millis();
        Self {
            original: OriginalExpiration::DateTime(OffsetDateTime::from_unix_timestamp(timestamp).unwrap())
        }
    }

    #[napi(getter)]
    pub fn is_session(&self) -> bool {
        self.original.is_session()
    }

    #[napi(getter)]
    pub fn is_datetime(&self) -> bool {
        self.original.is_datetime()
    }

    #[napi(getter)]
    pub fn datetime(&self) -> Option<DateTime<Utc>> {
        match &self.original {
            OriginalExpiration::DateTime(offset) => {
                let timestamp = offset.unix_timestamp();
                let datetime = DateTime::from_timestamp_millis(timestamp);
                datetime
            },
            _ => None
        }
    }
}

impl FromNapiValue for Expiration {
    unsafe fn from_napi_value(raw_env: napi::sys::napi_env, napi_val: napi::sys::napi_value) -> napi::Result<Self> {
        let expiration: &Expiration = Expiration::from_napi_ref(raw_env, napi_val)?;
        Ok(expiration.clone())
    }
}