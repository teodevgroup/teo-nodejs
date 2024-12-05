use napi::bindgen_prelude::{block_on, execute_tokio_future};
use napi::{Env, Error, JsFunction, JsObject, Result};
use napi::threadsafe_function::{ErrorStrategy, ThreadSafeCallContext, ThreadsafeFunction};
use teo::prelude::pipeline::item::templates::validator::Validity;
use teo::prelude::{Middleware, Next};
use teo::prelude::Request as TeoRequest;
use crate::dynamic::JSClassLookupMap;
use crate::middleware::SendMiddlewareCallback;
use crate::pipeline::ctx::PipelineCtx;
use crate::pipeline::item::compare_item_imp::PipelineCompareItemImp;
use crate::pipeline::item::item_imp::PipelineItemImp;
use teo::prelude::namespace;
use teo::prelude::model::field;
use teo::prelude::handler;
use teo::prelude::{r#enum, Value as TeoValue, Arguments, Arguments as TeoArgs, pipeline, model, transaction, response::Response as TeoResponse};
use crate::handler::group::HandlerGroup;
use crate::model::model::Model;
use crate::model::relation::relation::Relation;
use crate::model::field::field::Field;
use crate::model::property::property::Property;
use crate::object::promise::TeoValueOrPromise;
use crate::object::arguments::teo_args_to_js_args;
use crate::object::value::teo_value_to_js_any;
use crate::r#enum::member::member::EnumMember;
use crate::r#enum::r#enum::Enum;
use crate::request::Request;
use crate::response::ResponseOrPromise;

#[napi(js_name = "Namespace")]
pub struct Namespace {
    pub(crate) builder: namespace::Builder
}

#[napi]
impl Namespace {

    #[napi(js_name = "isMain")]
    pub fn is_main(&self) -> bool {
        self.builder.is_main()
    }

    #[napi(js_name = "isStd")]
    pub fn is_std(&self) -> bool {
        self.builder.is_std()
    }

    #[napi]
    pub fn path(&self) -> &Vec<String> {
        self.builder.path()
    }

    #[napi]
    pub fn namespace(&self, name: String) -> Option<Namespace> {
        self.builder.child_namespace(name.as_str()).map(|n| Namespace { 
            builder: n,
        })
    }

    #[napi]
    pub fn namespace_or_create(&self, name: String) -> Namespace {
        Namespace{ 
            builder: self.builder.child_namespace_or_create(name.as_str()),
        }
    }

    #[napi]
    pub fn namespace_at_path(&self, path: Vec<String>) -> Option<Namespace> {
        self.builder.descendant_namespace_at_path(&path).map(|n| Namespace { 
            builder: n,
        })
    }

    #[napi]
    pub fn namespace_or_create_at_path(&self, path: Vec<String>) -> Namespace {
        Namespace {
            builder: self.builder.descendant_namespace_or_create_at_path(&path),
        }
    }

    #[napi(js_name = "defineModelDecorator", ts_args_type = "name: string, body: (args: {[key: string]: any}, model: Model) => void")]
    pub fn define_model_decorator(&self, name: String, callback: JsFunction) -> Result<()> {
        let lookup_map = JSClassLookupMap::from_app_data(self.builder.app_data());
        let threadsafe_callback: ThreadsafeFunction<(teo::prelude::Arguments, model::Builder), ErrorStrategy::Fatal> = callback.create_threadsafe_function(0, move |ctx: ThreadSafeCallContext<(Arguments, model::Builder)>| {
            let arguments = teo_args_to_js_args(lookup_map, &ctx.value.0, &ctx.env)?;
            let js_model = Model { builder: ctx.value.1 };
            Ok(vec![arguments, js_model.into_instance(ctx.env)?.as_object(ctx.env)])
        })?;
        self.builder.define_model_decorator(name.as_str(), move |arguments, model| {
            let threadsafe_callback = threadsafe_callback.clone();
            let _ = threadsafe_callback.call((arguments, model.clone()), napi::threadsafe_function::ThreadsafeFunctionCallMode::Blocking);
            Ok(())
        });
        Ok(())
    }

    #[napi(js_name = "defineModelFieldDecorator", ts_args_type = "name: string, body: (args: {[key: string]: any}, field: Field) => void")]
    pub fn define_model_field_decorator(&self, name: String, callback: JsFunction) -> Result<()> {
        let lookup_map = JSClassLookupMap::from_app_data(self.builder.app_data());
        let threadsafe_callback: ThreadsafeFunction<(teo::prelude::Arguments, model::field::Builder), ErrorStrategy::Fatal> = callback.create_threadsafe_function(0, |ctx: ThreadSafeCallContext<(Arguments, field::Builder)>| {
            let arguments = teo_args_to_js_args(lookup_map, &ctx.value.0, &ctx.env)?;
            let js_model = Field { builder: ctx.value.1 };
            Ok(vec![arguments, js_model.into_instance(ctx.env)?.as_object(ctx.env)])
        })?;
        self.builder.define_model_field_decorator(name.as_str(), move |arguments, field| {
            let threadsafe_callback = threadsafe_callback.clone();
            let _ = threadsafe_callback.call((arguments, field.clone()), napi::threadsafe_function::ThreadsafeFunctionCallMode::Blocking);
            Ok(())
        });
        Ok(())
    }

    #[napi(js_name = "defineModelRelationDecorator", ts_args_type = "name: string, body: (args: {[key: string]: any}, relation: Relation) => void")]
    pub fn define_model_relation_decorator(&self, name: String, callback: JsFunction) -> Result<()> {
        let lookup_map = JSClassLookupMap::from_app_data(self.builder.app_data());
        let threadsafe_callback: ThreadsafeFunction<(teo::prelude::Arguments, model::relation::Builder), ErrorStrategy::Fatal> = callback.create_threadsafe_function(0, |ctx: ThreadSafeCallContext<(Arguments, model::relation::Builder)>| {
            let arguments = teo_args_to_js_args(lookup_map, &ctx.value.0, &ctx.env)?;
            let js_model = Relation { builder: ctx.value.1 };
            Ok(vec![arguments, js_model.into_instance(ctx.env)?.as_object(ctx.env)])
        })?;
        self.builder.define_model_relation_decorator(name.as_str(), move |arguments, relation_builder| {
            let threadsafe_callback = threadsafe_callback.clone();
            let _ = threadsafe_callback.call((arguments, relation_builder.clone()), napi::threadsafe_function::ThreadsafeFunctionCallMode::Blocking);
            Ok(())
        });
        Ok(())
    }

    #[napi(js_name = "defineModelPropertyDecorator", ts_args_type = "name: string, body: (args: {[key: string]: any}, property: Property) => void")]
    pub fn define_model_property_decorator(&self, name: String, callback: JsFunction) -> Result<()> {
        let lookup_map = JSClassLookupMap::from_app_data(self.builder.app_data());
        let threadsafe_callback: ThreadsafeFunction<(teo::prelude::Arguments, model::property::Builder), ErrorStrategy::Fatal> = callback.create_threadsafe_function(0, |ctx: ThreadSafeCallContext<(Arguments, model::property::Builder)>| {
            let arguments = teo_args_to_js_args(lookup_map, &ctx.value.0, &ctx.env)?;
            let js_model = Property { builder: ctx.value.1 };
            Ok(vec![arguments, js_model.into_instance(ctx.env)?.as_object(ctx.env)])
        })?;
        self.builder.define_model_property_decorator(name.as_str(), move |arguments, property_builder| {
            let threadsafe_callback = threadsafe_callback.clone();
            let _ = threadsafe_callback.call((arguments, property_builder.clone()), napi::threadsafe_function::ThreadsafeFunctionCallMode::Blocking);
            Ok(())
        });
        Ok(())
    }

    #[napi(js_name = "defineEnumDecorator", ts_args_type = "name: string, body: (args: {[key: string]: any}, e: Enum) => void")]
    pub fn define_enum_decorator(&self, name: String, callback: JsFunction) -> Result<()> {
        let lookup_map = JSClassLookupMap::from_app_data(self.builder.app_data());
        let threadsafe_callback: ThreadsafeFunction<(teo::prelude::Arguments, r#enum::Builder), ErrorStrategy::Fatal> = callback.create_threadsafe_function(0, |ctx: ThreadSafeCallContext<(Arguments, r#enum::Builder)>| {
            let arguments = teo_args_to_js_args(lookup_map, &ctx.value.0, &ctx.env)?;
            let js_model = Enum { builder: ctx.value.1 };
            Ok(vec![arguments, js_model.into_instance(ctx.env)?.as_object(ctx.env)])
        })?;
        self.builder.define_enum_decorator(name.as_str(), move |arguments, enum_builder| {
            let threadsafe_callback = threadsafe_callback.clone();
            let _ = threadsafe_callback.call((arguments, enum_builder.clone()), napi::threadsafe_function::ThreadsafeFunctionCallMode::Blocking);
            Ok(())
        });
        Ok(())
    }

    #[napi(js_name = "defineEnumMemberDecorator", ts_args_type = "name: string, body: (args: {[key: string]: any}, member: EnumMember) => void")]
    pub fn define_enum_member_decorator(&self, name: String, callback: JsFunction) -> Result<()> {
        let lookup_map = JSClassLookupMap::from_app_data(self.builder.app_data());
        let threadsafe_callback: ThreadsafeFunction<(teo::prelude::Arguments, r#enum::member::Builder), ErrorStrategy::Fatal> = callback.create_threadsafe_function(0, |ctx: ThreadSafeCallContext<(Arguments, r#enum::member::Builder)>| {
            let arguments = teo_args_to_js_args(lookup_map, &ctx.value.0, &ctx.env)?;
            let js_model = EnumMember { builder: ctx.value.1 };
            Ok(vec![arguments, js_model.into_instance(ctx.env)?.as_object(ctx.env)])
        })?;
        self.builder.define_enum_member_decorator(name.as_str(), move |arguments, enum_member_builder| {
            let threadsafe_callback = threadsafe_callback.clone();
            let _ = threadsafe_callback.call((arguments, enum_member_builder.clone()), napi::threadsafe_function::ThreadsafeFunctionCallMode::Blocking);
            Ok(())
        });
        Ok(())
    }

    #[napi(ts_args_type = "name: string, creator: (args: {[key: string]: any}) => (ctx: PipelineCtx) => any | Promise<any>")]
    pub fn _define_pipeline_item(&self, name: String, creator: JsFunction) -> Result<()> {
        let lookup_map = JSClassLookupMap::from_app_data(self.builder.app_data());
        let threadsafe_creator: ThreadsafeFunction<Arguments, ErrorStrategy::CalleeHandled> = creator.create_threadsafe_function(0, |ctx: ThreadSafeCallContext<Arguments>| {
            let js_args = teo_args_to_js_args(lookup_map, &ctx.value, &ctx.env)?;
            Ok(vec![js_args])
        })?;
        self.builder.define_pipeline_item(&name, move |arguments: Arguments| {
            let threadsafe_creator = threadsafe_creator.clone();
            let item = block_on(async move {
                let item: PipelineItemImp = threadsafe_creator.call_async(Ok(arguments)).await?;
                Ok::<PipelineItemImp, Error>(item)
            })?;
            Ok(move |ctx: pipeline::Ctx| {
                let item = item.clone();
                async move {
                    let result: TeoValueOrPromise = item.threadsafe_function.call_async(Ok(ctx)).await?;
                    let value = result.to_teo_value().await?;
                    Ok(value)
                }
            })
        });
        Ok(())
    }

    pub fn _define_validator_pipeline_item(&self, name: String, creator: JsFunction) -> Result<()> {
        let lookup_map = JSClassLookupMap::from_app_data(self.builder.app_data());
        let threadsafe_creator: ThreadsafeFunction<Arguments, ErrorStrategy::CalleeHandled> = creator.create_threadsafe_function(0, |ctx: ThreadSafeCallContext<Arguments>| {
            let js_args = teo_args_to_js_args(lookup_map, &ctx.value, &ctx.env)?;
            Ok(vec![js_args])
        })?;
        self.builder.define_pipeline_item(&name, move |arguments: Arguments| {
            let threadsafe_creator = threadsafe_creator.clone();
            let item = block_on(async move {
                let item: PipelineItemImp = threadsafe_creator.call_async(Ok(arguments)).await?;
                Ok::<PipelineItemImp, Error>(item)
            })?;
            Ok(move |ctx: pipeline::Ctx| {
                let item = item.clone();
                async move {
                    let result: TeoValueOrPromise = item.threadsafe_function.call_async(Ok(ctx.clone())).await?;
                    let value = result.to_teo_value().await?;
                    match value {
                        TeoValue::String(s) => Err(teo_result::Error::new_with_code(s, 400)),
                        TeoValue::Bool(b) => if b {
                            Ok(ctx.value().clone())
                        } else {
                            Err(teo_result::Error::new_with_code("value is invalid".to_owned(), 400))
                        },
                        _ => Ok(ctx.value().clone())
                    }
                }
            })
        });
        Ok(())
    }

    pub fn _define_callback_pipeline_item(&self, name: String, creator: JsFunction) -> Result<()> {
        let lookup_map = JSClassLookupMap::from_app_data(self.builder.app_data());
        let threadsafe_creator: ThreadsafeFunction<Arguments, ErrorStrategy::CalleeHandled> = creator.create_threadsafe_function(0, |ctx: ThreadSafeCallContext<Arguments>| {
            let js_args = teo_args_to_js_args(lookup_map, &ctx.value, &ctx.env)?;
            Ok(vec![js_args])
        })?;
        self.builder.define_pipeline_item(&name, move |arguments: Arguments| {
            let threadsafe_creator = threadsafe_creator.clone();
            let item = block_on(async move {
                let item: PipelineItemImp = threadsafe_creator.call_async(Ok(arguments)).await?;
                Ok::<PipelineItemImp, Error>(item)
            })?;
            Ok(move |ctx: pipeline::Ctx| {
                let item = item.clone();
                async move {
                    let result: TeoValueOrPromise = item.threadsafe_function.call_async(Ok(ctx.clone())).await?;
                    let _ = result.to_teo_value().await?;
                    Ok(ctx.value().clone())
                }
            })
        });
        Ok(())
    }

    pub fn _define_compare_pipeline_item(&self, name: String, creator: JsFunction) -> Result<()> {
        let lookup_map = JSClassLookupMap::from_app_data(self.builder.app_data());
        let threadsafe_creator: ThreadsafeFunction<Arguments, ErrorStrategy::CalleeHandled> = creator.create_threadsafe_function(0, |ctx: ThreadSafeCallContext<Arguments>| {
            let js_args = teo_args_to_js_args(lookup_map, &ctx.value, &ctx.env)?;
            Ok(vec![js_args])
        })?;
        self.builder.define_pipeline_item(&name, move |arguments: Arguments| {
            let threadsafe_creator = threadsafe_creator.clone();
            let item = block_on(async move {
                let item: PipelineCompareItemImp = threadsafe_creator.call_async(Ok(arguments)).await?;
                Ok::<PipelineCompareItemImp, Error>(item)
            })?;
            Ok(move |ctx: pipeline::Ctx| {
                let item = item.clone();
                async move {
                    if ctx.object().is_new() {
                        return Ok(ctx.value().clone());
                    }
                    let key = ctx.path()[ctx.path().len() - 1].as_key().unwrap();
                    let previous_value = ctx.object().get_previous_value(key)?;
                    let current_value = ctx.value();
                    if &previous_value == current_value {
                        return Ok(ctx.value().clone());
                    }
                    let result: TeoValueOrPromise = item.threadsafe_function.call_async(Ok((previous_value, current_value.clone(), ctx.clone()))).await?;
                    let _ = result.to_teo_value().await?;
                    Ok(ctx.value().clone())
                }
            })
        });
        Ok(())
    }

    #[napi(js_name = "_defineHandler", ts_args_type = "name: string, callback: (request: Request) => Response | Promise<Response>")]
    pub fn define_handler(&self, name: String, callback: JsFunction) -> Result<()> {
        let threadsafe_callback: ThreadsafeFunction<TeoRequest, ErrorStrategy::CalleeHandled> = callback.create_threadsafe_function(0, |ctx: ThreadSafeCallContext<TeoRequest>| {
            let request_ctx = Request::from(ctx.value);
            let request_ctx_instance = request_ctx.into_instance(ctx.env)?;
            let request_ctx_unknown = request_ctx_instance.as_object(ctx.env).into_unknown();
            Ok(vec![request_ctx_unknown])
        })?;
        self.builder.define_handler(name.as_str(), move |ctx: TeoRequest| {
            let threadsafe_callback = threadsafe_callback.clone();
            async move {
                let response_unknown: ResponseOrPromise = threadsafe_callback.call_async(Ok(ctx)).await?;
                Ok::<TeoResponse, teo::prelude::Error>(response_unknown.to_teo_response().await?)    
            }
        });
        Ok(())
    }

    #[napi(js_name = "defineHandlerGroup", ts_args_type = "name: string, callback: (group: HandlerGroup) => void")]
    pub fn define_handler_group(&self, name: String, callback: JsFunction) -> Result<()> {
        let threadsafe_callback: ThreadsafeFunction<HandlerGroup, ErrorStrategy::Fatal> = callback.create_threadsafe_function(0, |ctx: ThreadSafeCallContext<HandlerGroup>| {
            let handler_group = ctx.value;
            Ok(vec![handler_group])
        })?;
        self.builder.define_handler_group(name.as_str(), move |teo_handler_group: &handler::group::Builder| {
            let threadsafe_callback = threadsafe_callback.clone();
            let handler_group = HandlerGroup { builder: teo_handler_group.clone() };
            let _ = threadsafe_callback.call(handler_group, napi::threadsafe_function::ThreadsafeFunctionCallMode::Blocking);
            Ok(())
        })?;
        Ok(())
    }

    #[napi(js_name = "defineModelHandlerGroup", ts_args_type = "name: string, callback: (group: HandlerGroup) => void")]
    pub fn define_model_handler_group(&self, name: String, callback: JsFunction) -> Result<()> {
        let threadsafe_callback: ThreadsafeFunction<HandlerGroup, ErrorStrategy::Fatal> = callback.create_threadsafe_function(0, |ctx: ThreadSafeCallContext<HandlerGroup>| {
            let handler_group = ctx.value;
            Ok(vec![handler_group])
        })?;
        self.builder.define_model_handler_group(name.as_str(), move |teo_handler_group: &handler::group::Builder| {
            let threadsafe_callback = threadsafe_callback.clone();
            let handler_group = HandlerGroup { builder: teo_handler_group.clone() };
            let _ = threadsafe_callback.call(handler_group, napi::threadsafe_function::ThreadsafeFunctionCallMode::Blocking);
            Ok(())
        })?;
        Ok(())
    }

    #[napi(js_name = "defineRequestMiddleware", ts_args_type = "name: string, creator: (args: {[key: string]: any}) => (request: Request, next: (request: Request) => Promise<Response>) => Promise<Response> | Response")]
    pub fn define_request_middleware(&self, name: String, creator: JsFunction) -> Result<()> {
        let lookup_map = JSClassLookupMap::from_app_data(self.builder.app_data());
        let threadsafe_creator: ThreadsafeFunction<Arguments, ErrorStrategy::Fatal> = creator.create_threadsafe_function(0, |ctx: ThreadSafeCallContext<Arguments>| {
            let js_args = teo_args_to_js_args(lookup_map, &ctx.value, &ctx.env)?;
            Ok(vec![js_args])
        })?;
        self.builder.define_request_middleware(name.as_str(), move |arguments| {
            let threadsafe_creator = threadsafe_creator.clone();
            async move {
                let middleware_function: SendMiddlewareCallback = threadsafe_creator.call_async(arguments).await?;
                let wrapped_result = move |request: TeoRequest, next: Next| {
                    let middleware_function = middleware_function.clone();
                    async move {
                        let res_or_promise: ResponseOrPromise = middleware_function.inner.call_async((request, next)).await?;
                        let res = res_or_promise.to_teo_response().await?;
                        return Ok(res);    
                    }
                };
                return Ok(Middleware::new(wrapped_result));    
            }
        });
        Ok(())
    }

    #[napi(js_name = "defineHandlerMiddleware", ts_args_type = "name: string, creator: (args: {[key: string]: any}) => (request: Request, next: (request: Request) => Promise<Response>) => Promise<Response> | Response")]
    pub fn define_handler_middleware(&self, name: String, creator: JsFunction) -> Result<()> {
        let lookup_map = JSClassLookupMap::from_app_data(self.builder.app_data());
        let threadsafe_creator: ThreadsafeFunction<Arguments, ErrorStrategy::Fatal> = creator.create_threadsafe_function(0, |ctx: ThreadSafeCallContext<Arguments>| {
            let js_args = teo_args_to_js_args(lookup_map, &ctx.value, &ctx.env)?;
            Ok(vec![js_args])
        })?;
        self.builder.define_handler_middleware(name.as_str(), move |arguments| {
            let threadsafe_creator = threadsafe_creator.clone();
            async move {
                let middleware_function: SendMiddlewareCallback = threadsafe_creator.call_async(arguments).await?;
                let wrapped_result = move |request: TeoRequest, next: Next| {
                    let middleware_function = middleware_function.clone();
                    async move {
                        let res_or_promise: ResponseOrPromise = middleware_function.inner.call_async((request, next)).await?;
                        let res = res_or_promise.to_teo_response().await?;
                        return Ok(res);    
                    }
                };
                return Ok(Middleware::new(wrapped_result));    
            }
        });
        Ok(())
    }
}
