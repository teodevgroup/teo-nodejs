use napi::{threadsafe_function::{ErrorStrategy, ThreadSafeCallContext, ThreadsafeFunction}, JsFunction, JsString, JsUnknown, Result};
use teo::prelude::{handler, Response as OriginalResponse, Request as OriginalRequest};

use crate::{request::Request, response::response_or_promise::ResponseOrPromise};

#[napi(js_name = "HandlerGroup")]
pub struct HandlerGroup {
    pub(crate) builder: handler::group::Builder,
}

#[napi]
impl HandlerGroup {

    #[napi(js_name = "_defineHandler", ts_args_type = "name: string, callback: (request: Request) => Response | Promise<Response>")]
    pub fn _define_handler(&self, name: String, callback: JsFunction) -> Result<()> {
        let threadsafe_callback: ThreadsafeFunction<OriginalRequest, ErrorStrategy::CalleeHandled> = callback.create_threadsafe_function(0, |ctx: ThreadSafeCallContext<OriginalRequest>| {
            let request = Request::from(ctx.value);
            let request_instance = request.into_instance(ctx.env)?;
            let request_unknown = request_instance.as_object(ctx.env).into_unknown();
            Ok(vec![request_unknown])
        })?;
        self.builder.define_handler(name.as_str(), move |request: OriginalRequest| {
            let threadsafe_callback = threadsafe_callback.clone();
            async move {
                let response_unknown: ResponseOrPromise = threadsafe_callback.call_async(Ok(request)).await?;
                Ok::<OriginalResponse, teo::prelude::Error>(response_unknown.to_teo_response().await?)    
            }
        });
        Ok(())
    }
}