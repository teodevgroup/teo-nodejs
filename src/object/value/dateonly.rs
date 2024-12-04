use chrono::NaiveDate;
use napi::Result;
use teo::prelude::Error;

#[derive(Clone, Copy)]
#[napi]
pub struct DateOnly {
    pub(crate) original: NaiveDate
}

impl From<NaiveDate> for DateOnly {
    fn from(original: NaiveDate) -> Self {
        Self { original }
    }
}

#[napi]
impl DateOnly {

    #[napi(constructor)]
    pub fn constructor(string: String) -> Result<Self> {
        match NaiveDate::parse_from_str(string.as_str(), "%Y-%m-%d") {
            Ok(original) => Ok(Self::from(original)),
            Err(_) => {
                Err(Error::new("string doesn't represent valid DateOnly"))?
            }
        }
    }

    #[napi]
    pub fn to_string(&self) -> String {
        self.original.format("%Y-%m-%d").to_string()
    }
}