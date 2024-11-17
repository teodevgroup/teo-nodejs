use napi::{Result, JsFunction, threadsafe_function::{ThreadSafeCallContext, ErrorStrategy, ThreadsafeFunction}};
use teo::prelude::{handler, Response as TeoResponse};

use crate::{request::Request, response::response_or_promise::ResponseOrPromise};

#[napi(js_name = "HandlerGroup")]
pub struct HandlerGroup {
    pub(crate) builder: handler::group::Builder,
}

#[napi]
impl HandlerGroup {

    #[napi(js_name = "_defineHandler", ts_args_type = "name: string, callback: (request: Request) => Response | Promise<Response>")]
    pub fn define_handler(&'static mut self, name: String, callback: JsFunction) -> Result<()> {
        let tsfn: ThreadsafeFunction<teo::prelude::Request, ErrorStrategy::CalleeHandled> = callback.create_threadsafe_function(0, |ctx: ThreadSafeCallContext<teo::prelude::Request>| {
            let request_ctx = Request::new(ctx.value);
            let request_ctx_instance = request_ctx.into_instance(ctx.env)?;
            let request_ctx_unknown = request_ctx_instance.as_object(ctx.env).into_unknown();
            Ok(vec![request_ctx_unknown])
        })?;
        let tsfn_cloned = &*Box::leak(Box::new(tsfn));
        self.builder.define_handler(name.as_str(), move |ctx: teo::prelude::Request| async move {
            let response_unknown: ResponseOrPromise = tsfn_cloned.call_async(Ok(ctx)).await?;
            Ok::<TeoResponse, teo::prelude::Error>(response_unknown.to_teo_response().await?)
        });
        Ok(())
    }
}