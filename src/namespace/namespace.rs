use napi::bindgen_prelude::execute_tokio_future;
use napi::{Env, JsFunction, Result};
use napi::threadsafe_function::{ErrorStrategy, ThreadSafeCallContext, ThreadsafeFunction};
use teo::prelude::pipeline::item::templates::validator::Validity;
use teo::prelude::{Middleware, Next};
use teo::prelude::Request as TeoRequest;
use crate::dynamic::JSClassLookupMap;
use crate::middleware::SendMiddlewareCallback;
use crate::pipeline::ctx::PipelineCtx;
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

    // #[napi(js_name = "definePipelineItem", ts_args_type = "name: string, body: (input: any, args: {[key: string]: any}, object: any, teo: any) => any | Promise<any>")]
    // pub fn define_pipeline_item(&self, name: String, creator: JsFunction, env: Env) -> Result<()> {
    //     let lookup_map = JSClassLookupMap::from_app_data(self.builder.app_data());
    //     let threadsafe_creator: ThreadsafeFunction<Arguments, ErrorStrategy::CalleeHandled> = creator.create_threadsafe_function(0, |ctx: ThreadSafeCallContext<Arguments>| {
    //         let js_args = teo_args_to_js_args(lookup_map, &ctx.value, &ctx.env)?;
    //         Ok(vec![js_args])
    //     })?;
    //     self.builder.define_pipeline_item(name.as_str(), move |arguments: Arguments| {
    //         let threadsafe_creator = threadsafe_creator.clone();
    //         let promise: JsObject = env.execute_tokio_future(async move {
    //             let item: JsFunction = threadsafe_creator.call_async(Ok(arguments)).await?;
    //             Ok(item)
    //         }())?;

    //         Ok(move |ctx: pipeline::Ctx| {
    //             async move {

    //             }
    //         })
    //         async move {
    //             let object = ctx.value().clone();
    //             let model_object = ctx.object().clone();
    //             let transaction_ctx = ctx.transaction_ctx().clone();
    //             let result: TeoValueOrPromise = threadsafe_callback.call_async((object, args, model_object, transaction_ctx)).await?;
    //             Ok(result.to_teo_value().await?)    
    //         }
    //     });
    //     Ok(())
    // }

    // #[napi(js_name = "defineTransformPipelineItem", ts_args_type = "name: string, callback: (input: any, args: {[key: string]: any}, object: any, teo: any) => any | Promise<any>")]
    // pub fn define_transform_pipeline_item(&self, name: String, callback: JsFunction) -> Result<()> {
    //     self.define_pipeline_item(name, callback)
    // }

    // #[napi(ts_args_type = "name: string, callback: (input: any, args: {[key: string]: any}, object: any, teo: any) => boolean | string | undefined | null | Promise<boolean | string | undefined | null>")]
    // pub fn define_validator_pipeline_item(&self, name: String, callback: JsFunction) -> Result<()> {
    //     let lookup_map = JSClassLookupMap::from_app_data(self.builder.app_data());
    //     let threadsafe_callback: ThreadsafeFunction<(TeoValue, TeoArgs, model::Object, transaction::Ctx), ErrorStrategy::Fatal> = callback.create_threadsafe_function(0, |ctx: ThreadSafeCallContext<(TeoValue, TeoArgs, model::Object, transaction::Ctx)>| {
    //         let js_value = teo_value_to_js_any(lookup_map, &ctx.value.0, &ctx.env)?;
    //         let js_args = teo_args_to_js_args(lookup_map, &ctx.value.1, &ctx.env)?;
    //         let map = JSClassLookupMap::from_app_data(app_data);
    //         let js_object = map.teo_model_object_to_js_model_object_object(ctx.env, ctx.value.2.clone())?;
    //         let js_ctx = map.teo_transaction_ctx_to_js_ctx_object(ctx.env, ctx.value.3.clone(), "")?;
    //         Ok(vec![js_value, js_args.into_unknown(), js_object.into_unknown(), js_ctx.into_unknown()])
    //     })?;
    //     self.builder.define_validator_pipeline_item(name.as_str(), move |value: TeoValue, args: TeoArgs, ctx: pipeline::Ctx| {
    //         let threadsafe_callback = threadsafe_callback.clone();
    //         async move {
    //             let result: TeoValueOrPromise = threadsafe_callback.call_async((value, args, ctx.object().clone(), ctx.transaction_ctx())).await?;
    //             let teo_value = result.to_teo_value().await?;
    //             Ok::<Validity, teo::prelude::Error>(match teo_value {
    //                 TeoValue::String(s) => {
    //                     Validity::Invalid(s.to_owned())
    //                 },
    //                 TeoValue::Bool(b) => if b {
    //                     Validity::Valid
    //                 } else {
    //                     Validity::Invalid("value is invalid".to_owned())
    //                 },
    //                 _ => Validity::Valid
    //             })    
    //         }
    //     });
    //     Ok(())
    // }

    // /// Register a named callback.
    // #[napi(ts_args_type = "name: string, callback: (input: any, args: {[key: string]: any}, object: any, teo: any) => void | Promise<void>")]
    // pub fn define_callback_pipeline_item(&self, name: String, callback: JsFunction) -> Result<()> {
    //     let lookup_map = JSClassLookupMap::from_app_data(self.builder.app_data());
    //     let threadsafe_callback: ThreadsafeFunction<(TeoValue, TeoArgs, model::Object, transaction::Ctx), ErrorStrategy::Fatal> = callback.create_threadsafe_function(0, |ctx: ThreadSafeCallContext<(TeoValue, TeoArgs, model::Object, transaction::Ctx)>| {
    //         let js_value = teo_value_to_js_any(lookup_map, &ctx.value.0, &ctx.env)?;
    //         let js_args = teo_args_to_js_args(lookup_map, &ctx.value.1, &ctx.env)?;
    //         let js_object = lookup_map.teo_model_object_to_js_model_object_object(ctx.env, ctx.value.2.clone())?;
    //         let js_ctx = lookup_map.teo_transaction_ctx_to_js_ctx_object(ctx.env, ctx.value.3.clone(), "")?;
    //         Ok(vec![js_value, js_args.into_unknown(), js_object.into_unknown(), js_ctx.into_unknown()])
    //     })?;
    //     self.builder.define_callback_pipeline_item(name.as_str(), move |value: TeoValue, args: TeoArgs, ctx: pipeline::Ctx| {
    //         let threadsafe_callback = threadsafe_callback.clone();
    //         async move {
    //             let model_object = ctx.object().clone();
    //             let transaction_ctx = ctx.transaction_ctx().clone();
    //             let result: TeoValueOrPromise = threadsafe_callback.call_async((value, args, model_object, transaction_ctx)).await?;
    //             result.to_teo_value().await?;
    //             Ok(())    
    //         }
    //     });
    //     Ok(())
    // }

    // #[napi(js_name = "defineComparePipelineItem<T>", ts_args_type = "name: string, callback: (oldValue: T, newValue: T, args: {[key: string]: any}, object: any, teo: any) => boolean | string | undefined | null | Promise<boolean | string | undefined | null>")]
    // pub fn define_compare_pipeline_item(&self, name: String, callback: JsFunction) -> Result<()> {
    //     let lookup_map = JSClassLookupMap::from_app_data(self.builder.app_data());
    //     let threadsafe_callback: ThreadsafeFunction<(TeoValue, TeoValue, TeoArgs, model::Object, transaction::Ctx), ErrorStrategy::Fatal> = callback.create_threadsafe_function(0, |ctx: ThreadSafeCallContext<(TeoValue, TeoValue, TeoArgs, model::Object, transaction::Ctx)>| {
    //         let js_value_old = teo_value_to_js_any(lookup_map, &ctx.value.0, &ctx.env)?;
    //         let js_value_new = teo_value_to_js_any(lookup_map, &ctx.value.1, &ctx.env)?;
    //         let js_args = teo_args_to_js_args(lookup_map, &ctx.value.2, &ctx.env)?;
    //         let js_object = map.teo_model_object_to_js_model_object_object(ctx.env, ctx.value.3.clone())?;
    //         let js_ctx = map.teo_transaction_ctx_to_js_ctx_object(ctx.env, ctx.value.4.clone(), "")?;
    //         Ok(vec![js_value_old, js_value_new, js_args.into_unknown(), js_object.into_unknown(), js_ctx.into_unknown()])
    //     })?;
    //     self.builder.define_compare_pipeline_item(name.as_str(), move |old: TeoValue, new: TeoValue, args: TeoArgs, object: TeoValue, ctx: pipeline::Ctx| {
    //         let threadsafe_callback = threadsafe_callback.clone();
    //         async move {
    //             let result: TeoValueOrPromise = threadsafe_callback.call_async((old, new, args, ctx.object().clone(), ctx.transaction_ctx())).await?;
    //             let teo_value = result.to_teo_value().await?;
    //             Ok::<Validity, teo::prelude::Error>(match teo_value {
    //                 TeoValue::String(s) => {
    //                     Validity::Invalid(s.to_owned())
    //                 },
    //                 TeoValue::Bool(b) => if b {
    //                     Validity::Valid
    //                 } else {
    //                     Validity::Invalid("value is invalid".to_owned())
    //                 },
    //                 _ => Validity::Valid
    //             })    
    //         }
    //     });
    //     Ok(())
    // }

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
        });
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
        });
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
