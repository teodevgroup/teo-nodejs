use napi::{threadsafe_function::{ThreadsafeFunction, ErrorStrategy, ThreadSafeCallContext}, bindgen_prelude::FromNapiValue, JsFunction, Env};
use teo::prelude::{request, Response as TeoResponse};

use crate::{request::{send_next::SendNext, RequestCtx}, result::IntoNodeJSResult, response::Response};

pub struct SendMiddlewareCallback {
    pub(crate) inner: &'static ThreadsafeFunction<(request::Ctx, SendNext), ErrorStrategy::Fatal>
}

unsafe impl Send for SendMiddlewareCallback { }
unsafe impl Sync for SendMiddlewareCallback { }

impl FromNapiValue for SendMiddlewareCallback {
    unsafe fn from_napi_value(env: napi::sys::napi_env, napi_val: napi::sys::napi_value) -> napi::Result<Self> {
        let js_function = JsFunction::from_napi_value(env, napi_val)?;
        let thread_safe_function: ThreadsafeFunction<(teo::prelude::request::Ctx, SendNext), ErrorStrategy::Fatal> = js_function.create_threadsafe_function(0, |ctx: ThreadSafeCallContext<(teo::prelude::request::Ctx, SendNext)>| {
            let request_ctx = RequestCtx::new(ctx.value.0.clone());
            let request_ctx_unknown = request_ctx.into_instance(ctx.env)?.as_object(ctx.env).into_unknown();
            let wrapped_next = ctx.env.create_function_from_closure("next", move |_| {
                let teo_request_ctx = ctx.value.0.clone();
                let teo_next = ctx.value.1;
                let teo_next = teo_next.next();
                let promise = ctx.env.execute_tokio_future((move || async move {
                    let result = teo_next.call(teo_request_ctx).await?;
                    Ok(result)
                })(), |env: &mut Env, response: TeoResponse| {
                    Ok(Response {
                        teo_response: response.clone()
                    }.into_instance(*env)?.as_object(*env))
                })?;
                Ok(promise)
            })?.into_unknown();
            Ok(vec![request_ctx_unknown, wrapped_next])
        })?;
        let tsfn_cloned: &'static ThreadsafeFunction<(request::Ctx, SendNext), ErrorStrategy::Fatal> = &*Box::leak(Box::new(thread_safe_function));
        Ok(SendMiddlewareCallback { inner: tsfn_cloned })
    }
}