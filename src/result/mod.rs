use napi::Error;

pub trait IntoNodeJSResult<T> {
    fn into_nodejs_result(self) -> napi::Result<T>;
}

impl<T> IntoNodeJSResult<T> for teo::prelude::Result<T> {
    fn into_nodejs_result(self) -> napi::Result<T> {
        match self {
            Ok(r) => Ok(r),
            Err(e) => {
                if let Some(napi_error) = e.platform_native_object::<napi::Error>() {
                    // contains one native error, use it
                    Err(napi_error.clone())
                } else {
                    // TODO: convert to TeoError in Node.js
                    Err(Error::new(napi::Status::GenericFailure, e.message()))
                }
            }
        }
    }
}

pub trait IntoTeoResult<T> {
    fn into_teo_result(self) -> teo::prelude::Result<T>;
}

impl<T> IntoTeoResult<T> for napi::Result<T> {

    fn into_teo_result(self) -> teo::prelude::Result<T> {
        match self {
            Ok(r) => Ok(r),
            Err(e) => {
                let mut error = teo::prelude::Error::new(e.reason.as_str());
                error.assign_platform_native_object(e);
                Err(error)
            },
        }
    }
}
