use napi::{threadsafe_function::{ThreadsafeFunction, ErrorStrategy, ThreadSafeCallContext}, bindgen_prelude::FromNapiValue, JsFunction, Env};
use teo::prelude::{Next, NextImp, Response as TeoResponse};

use crate::{request::Request, response::Response};

#[derive(Clone)]
#[repr(transparent)]
pub struct SendMiddlewareCallback {
    pub(crate) inner: ThreadsafeFunction<(teo::prelude::Request, Next), ErrorStrategy::Fatal>,
}

unsafe impl Send for SendMiddlewareCallback { }
unsafe impl Sync for SendMiddlewareCallback { }

impl FromNapiValue for SendMiddlewareCallback {
    unsafe fn from_napi_value(env: napi::sys::napi_env, napi_val: napi::sys::napi_value) -> napi::Result<Self> {
        let callback = JsFunction::from_napi_value(env, napi_val)?;
        let threadsafe_callback: ThreadsafeFunction<(teo::prelude::Request, Next), ErrorStrategy::Fatal> = callback.create_threadsafe_function(0, |ctx: ThreadSafeCallContext<(teo::prelude::Request, Next)>| {
            let request_ctx = Request::new(ctx.value.0.clone());
            let request_ctx_unknown = request_ctx.into_instance(ctx.env)?.as_object(ctx.env).into_unknown();
            let teo_next = ctx.value.1;
            let wrapped_next = ctx.env.create_function_from_closure("next", move |_| {
                let teo_request = ctx.value.0.clone();
                let teo_next = teo_next.clone();
                let promise = ctx.env.execute_tokio_future((move || {
                    let teo_next = teo_next.clone();
                    async move {
                        let result = teo_next.call(teo_request).await?;
                        Ok(result)    
                    }
                })(), |env: &mut Env, response: TeoResponse| {
                    Ok(Response {
                        original: response.clone()
                    }.into_instance(*env)?.as_object(*env))
                })?;
                Ok(promise)
            })?.into_unknown();
            Ok(vec![request_ctx_unknown, wrapped_next])
        })?;
        Ok(SendMiddlewareCallback { inner: threadsafe_callback })
    }
}