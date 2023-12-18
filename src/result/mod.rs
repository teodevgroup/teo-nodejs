use napi::Error;

pub trait IntoNodeJSResult<T> {
    fn into_nodejs_result(self) -> napi::Result<T>;
}

impl<T> IntoNodeJSResult<T> for teo::prelude::Result<T> {
    fn into_nodejs_result(self) -> napi::Result<T> {
        match self {
            Ok(r) => Ok(r),
            Err(e) => Err(Error::from_reason(e.message())),
        }
    }
}