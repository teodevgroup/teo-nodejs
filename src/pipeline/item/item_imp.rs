use napi::{bindgen_prelude::FromNapiValue, sys::{napi_env, napi_value}, threadsafe_function::{ErrorStrategy, ThreadSafeCallContext, ThreadsafeFunction}, JsFunction, Result};
use teo::prelude::pipeline;
use crate::pipeline::ctx::PipelineCtx;

#[derive(Clone)]
#[repr(transparent)]
pub struct PipelineItemImp {
    pub(crate) threadsafe_function: ThreadsafeFunction<pipeline::Ctx, ErrorStrategy::CalleeHandled>,
}

impl FromNapiValue for PipelineItemImp {
    unsafe fn from_napi_value(raw_env: napi_env, napi_val: napi_value) -> Result<Self> {
        let function = JsFunction::from_napi_value(raw_env, napi_val)?;
        let threadsafe_function: ThreadsafeFunction<pipeline::Ctx, ErrorStrategy::CalleeHandled> = function.create_threadsafe_function(0, |ctx: ThreadSafeCallContext<pipeline::Ctx>| {
            let pipeline_ctx = PipelineCtx::from(ctx.value.clone());
            Ok(vec![pipeline_ctx])
        })?;
        Ok(Self { threadsafe_function })
    }
}
