use napi::{JsFunction, Result};
use napi::threadsafe_function::{ErrorStrategy, ThreadSafeCallContext, ThreadsafeFunction};
use teo::prelude::{Namespace as TeoNamespace, object::Object as TeoObject, Arguments as TeoArgs, pipeline};
use crate::object::promise::TeoObjectOrPromise;
use crate::object::teo_object_to_js_any;

#[napi(js_name = "Namespace")]
pub struct Namespace {
    pub(crate) teo_namespace: &'static mut TeoNamespace,
}

#[napi]
impl Namespace {

    #[napi(js_name = "definePipelineItem", ts_args_type = "name: string, body: (args: {[key: string]: any}, ctx?: any) => any | Promise<any>")]
    pub fn define_pipeline_item(&mut self, name: String, callback: JsFunction) -> Result<()> {
        let tsfn: ThreadsafeFunction<(TeoObject), ErrorStrategy::Fatal> = callback.create_threadsafe_function(0, |ctx: ThreadSafeCallContext<(TeoObject)>| {
            let js_value = teo_object_to_js_any(&ctx.value, &ctx.env)?;
            // let js_object = js_object_from_teo_object(ctx.env, ctx.value.1.clone())?;
            // let js_ctx = js_user_ctx_from_user_ctx(ctx.env, ctx.value.2.clone())?;
            Ok(vec![js_value])
            // , js_object.into_unknown(), js_ctx.into_unknown()
        })?;
        let tsfn_cloned = unsafe { &*Box::leak(Box::new(tsfn)) };
        self.teo_namespace.define_pipeline_item(name.as_str(), move |args: TeoArgs, ctx: pipeline::Ctx| async move {
            let object = ctx.value().clone();
            let result: TeoObjectOrPromise = tsfn_cloned.call_async(object).await.unwrap();
            Ok(result.to_teo_object().await.unwrap())
        });
        Ok(())
    }
}

