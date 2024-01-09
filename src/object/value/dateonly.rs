use chrono::NaiveDate;
use napi::{Env, JsUnknown, Result};

#[napi(js_name = "DateOnly")]
pub struct DateOnly {
    pub(crate) value: NaiveDate
}

#[napi]
impl DateOnly {

    #[napi]
    pub fn toString(&self) -> String {
        self.value.format("%Y-%m-%d").to_string()
    }

    #[napi]
    pub fn fromString(string: String, env: Env) -> Result<JsUnknown> {
        match NaiveDate::parse_from_str(string.as_str(), "%Y-%m-%d") {
            Ok(value) => Ok(Self { value }.into_instance(env)?.as_object(env).into_unknown()),
            Err(e) => {
                env.throw_type_error("string doesn't represent valid DateOnly", None)?;
                Ok(env.get_undefined()?.into_unknown())
            }
        }
    }
}