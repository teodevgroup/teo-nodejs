use napi::Error;

pub trait IntoNodeJSResult<T> {
    fn into_nodejs_result(self) -> napi::Result<T>;
}

impl<T> IntoNodeJSResult<T> for teo::prelude::path::Result<T> {
    fn into_nodejs_result(self) -> napi::Result<T> {
        match self {
            Ok(r) => Ok(r),
            Err(e) => {
                if let Some(napi_error) = e.get_meta::<napi::Error>("nodejs") {
                    Err(napi_error.clone())
                } else {
                    Err(Error::new(napi::Status::GenericFailure, e.message()))
                }
            }
        }
    }
}

impl<T> IntoNodeJSResult<T> for teo::prelude::Result<T> {
    fn into_nodejs_result(self) -> napi::Result<T> {
        match self {
            Ok(r) => Ok(r),
            Err(e) => {
                if let Some(napi_error) = e.get_meta::<napi::Error>("nodejs") {
                    Err(napi_error.clone())
                } else {
                    let (status, reason) = parse_error_status_and_reason(e.message());
                    Err(Error::new(status, reason))    
                }
            },
        }
    }
}

fn parse_error_status_and_reason(msg: &str) -> (napi::Status, String) {
    let msg_owned = if let Some(position) = msg.chars().position(|c| c == ':') {
        msg[(position + 2)..].to_owned()
    } else {
        msg.to_owned()
    };
    (napi::Status::GenericFailure, msg_owned)
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
                error.insert_meta("nodejs", e);
                Err(error)
            },
        }
    }
}

pub trait IntoTeoPathResult<T> {
    fn into_teo_path_result(self) -> teo::prelude::path::Result<T>;
}

impl<T> IntoTeoPathResult<T> for napi::Result<T> {
    fn into_teo_path_result(self) -> teo::prelude::path::Result<T> {
        match self {
            Ok(r) => Ok(r),
            Err(e) => {
                println!("herre error");
                let mut error = teo::prelude::path::Error::internal_server_error_message_only(e.reason.as_str());
                error.insert_meta("nodejs", e);
                Err(error)
            },
        }
    }
}