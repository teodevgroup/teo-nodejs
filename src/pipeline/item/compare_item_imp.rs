use napi::{bindgen_prelude::FromNapiValue, sys::{napi_env, napi_value}, threadsafe_function::{ErrorStrategy, ThreadSafeCallContext, ThreadsafeFunction}, JsFunction, Result};
use teo::prelude::{pipeline, Value as TeoValue};
use crate::{dynamic::DynamicClasses, object::value::teo_value_to_js_any, pipeline::ctx::PipelineCtx};

#[derive(Clone)]
#[repr(transparent)]
pub struct PipelineCompareItemImp {
    pub(crate) threadsafe_function: ThreadsafeFunction<(TeoValue, TeoValue, pipeline::Ctx), ErrorStrategy::CalleeHandled>,
}

impl FromNapiValue for PipelineCompareItemImp {
    unsafe fn from_napi_value(raw_env: napi_env, napi_val: napi_value) -> Result<Self> {
        let function = JsFunction::from_napi_value(raw_env, napi_val)?;
        let threadsafe_function: ThreadsafeFunction<(TeoValue, TeoValue, pipeline::Ctx), ErrorStrategy::CalleeHandled> = function.create_threadsafe_function(0, |ctx: ThreadSafeCallContext<(TeoValue, TeoValue, pipeline::Ctx)>| {
            let pipeline_ctx = PipelineCtx::from(ctx.value.2.clone());
            let dynamic_classes = DynamicClasses::retrieve(pipeline_ctx.original().object().namespace().app_data())?;
            let old_value = teo_value_to_js_any(&dynamic_classes, &ctx.value.0, &ctx.env)?;
            let new_value = teo_value_to_js_any(&dynamic_classes, &ctx.value.1, &ctx.env)?;
            let pipeline_ctx_instance = pipeline_ctx.into_instance(ctx.env)?;
            let pipeline_ctx_unknown = pipeline_ctx_instance.as_object(ctx.env).into_unknown();
            Ok(vec![old_value, new_value, pipeline_ctx_unknown])
        })?;
        Ok(Self { threadsafe_function })
    }
}
