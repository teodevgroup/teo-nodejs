use napi::bindgen_prelude::FromNapiRef;
use napi::{JsFunction, Result, Env, JsUnknown, JsObject, Error};
use napi::threadsafe_function::{ErrorStrategy, ThreadSafeCallContext, ThreadsafeFunction};
use teo::prelude::{Namespace as TeoNamespace, object::Object as TeoObject, Arguments as TeoArgs, pipeline, model, transaction, request, response::Response as TeoResponse};
use crate::dynamic::{js_ctx_object_from_teo_transaction_ctx, js_model_object_from_teo_model_object};
use crate::object::promise::TeoObjectOrPromise;
use crate::object::teo_object_to_js_any;
use crate::object::arguments::teo_args_to_js_args;
use crate::object::value::teo_value_to_js_any;
use crate::request::Request;
use crate::response::Response;

#[napi(js_name = "Namespace")]
pub struct Namespace {
    pub(crate) teo_namespace: &'static mut TeoNamespace,
}

#[napi]
impl Namespace {

    #[napi(js_name = "definePipelineItem", ts_args_type = "name: string, body: (value: any, args?: {[key: string]: any}, object?: any, ctx?: any) => any | Promise<any>")]
    pub fn define_pipeline_item(&mut self, name: String, callback: JsFunction) -> Result<()> {
        let tsfn: ThreadsafeFunction<(TeoObject, TeoArgs, model::Object, transaction::Ctx), ErrorStrategy::Fatal> = callback.create_threadsafe_function(0, |ctx: ThreadSafeCallContext<(TeoObject, TeoArgs, model::Object, transaction::Ctx)>| {
            let js_value = teo_object_to_js_any(&ctx.value.0, &ctx.env)?;
            let js_args = teo_args_to_js_args(&ctx.value.1, &ctx.env)?;
            let js_object = js_model_object_from_teo_model_object(ctx.env, ctx.value.2.clone())?;
            let js_ctx = js_ctx_object_from_teo_transaction_ctx(ctx.env, ctx.value.3.clone(), "")?;
            Ok(vec![js_value, js_args.into_unknown(), js_object.into_unknown(), js_ctx.into_unknown()])
        })?;
        let tsfn_cloned = unsafe { &*Box::leak(Box::new(tsfn)) };
        self.teo_namespace.define_pipeline_item(name.as_str(), move |args: TeoArgs, ctx: pipeline::Ctx| async move {
            let object = ctx.value().clone();
            let model_object = ctx.object().clone();
            let transaction_ctx = ctx.transaction_ctx().clone();
            let result: TeoObjectOrPromise = tsfn_cloned.call_async((object, args, model_object, transaction_ctx)).await.unwrap();
            Ok(result.to_teo_object().await.unwrap())
        });
        Ok(())
    }

    #[napi(js_name = "defineHandler")]
    pub fn define_handler(&mut self, name: String, callback: JsFunction) -> Result<()> {
        let tsfn: ThreadsafeFunction<request::Ctx, ErrorStrategy::Fatal> = callback.create_threadsafe_function(0, |ctx: ThreadSafeCallContext<request::Ctx>| {
            let teo_request = ctx.value.request().clone();
            let request = Request::new(teo_request);
            let request_instance = request.into_instance(ctx.env)?;
            let request_unknown = request_instance.as_object(ctx.env).into_unknown();
            let body = teo_value_to_js_any(&ctx.value.body(), &ctx.env)?;
            let js_ctx = js_ctx_object_from_teo_transaction_ctx(ctx.env, ctx.value.transaction_ctx(), "")?.into_unknown();
            Ok(vec![body, js_ctx, request_unknown])
        })?;
        let tsfn_cloned = unsafe { &*Box::leak(Box::new(tsfn)) };
        self.teo_namespace.define_handler(name.as_str(), move |ctx: request::Ctx| async move {
            let response_unknown: Response = tsfn_cloned.call_async(ctx).await.unwrap();
            Ok::<TeoResponse, teo::prelude::path::Error>(response_unknown.teo_response.clone())
        });
        Ok(())
    }
}

