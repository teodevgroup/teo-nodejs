#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

pub mod value;

use std::collections::HashMap;
use std::sync::Arc;
use napi::threadsafe_function::{ThreadsafeFunction, ErrorStrategy, ThreadSafeCallContext};
use napi::{Env, JsObject, JsString, JsFunction, Result, JsUndefined, CallContext, Property, JsUnknown, Error};
use napi::bindgen_prelude::Promise;
use napi::ValueType::Object;
use teo::core::app::{builder::AppBuilder, entrance::Entrance};
use teo::core::object::{Object as TeoObject};
use teo::core::graph::Graph;
use teo::core::pipeline::items::function::validate::{ValidateResult, Validity};
use teo::core::teon::Value as TeoValue;
use to_mut::ToMut;
use value::{teo_value_to_js_unknown, WrappedTeoValue};
use crate::value::js_unknown_to_teo_value;

static mut CLASSES: Option<&'static HashMap<String, napi::Ref<()>>> = None;

fn classes_mut() -> &'static mut HashMap<String, napi::Ref<()>> {
    unsafe {
        let const_ptr = CLASSES.unwrap() as *const HashMap<String, napi::Ref<()>>;
        let mut_ptr = const_ptr as *mut HashMap<String, napi::Ref<()>>;
        &mut *mut_ptr
    }
}

fn model_constructor_function(env: Env, name: String) -> Result<JsFunction> {
    let ctor = env.create_function_from_closure(&name, |ctx| {
        // let this = ctx.this_unchecked();
        ctx.env.get_undefined()
    })?;
    let prototype = env.create_object()?;
    let mut ctor_object = ctor.coerce_to_object()?;
    ctor_object.set_named_property("prototype", prototype)?;
    let r = env.create_reference(ctor_object)?;
    classes_mut().insert(name.clone(), r);
    let ref_get = unsafe { CLASSES.unwrap().get(name.as_str()).unwrap() };
    let object: JsFunction = env.get_reference_value(ref_get)?;
    Ok(object)
}

#[napi]
pub fn get_model_class(env: Env, name: String) -> JsFunction {
    unsafe {
        if let Some(object_ref) = CLASSES.unwrap().get(name.as_str()) {
            let object: JsFunction = env.get_reference_value(object_ref).unwrap();
            object
        } else {
            model_constructor_function(env, name).unwrap()
        }
    }
}

fn get_model_prototype(env: Env, name: String) -> JsObject {
    let js_function = get_model_class(env, name);
    let js_object = js_function.coerce_to_object().unwrap();
    let prototype: JsObject = js_object.get_named_property("prototype").unwrap();
    prototype
}

pub(crate) fn js_object_from_teo_object(env: &mut Env, teo_object: TeoObject) -> Result<JsObject> {
    let mut js_object = env.create_object()?;
    js_object.set_named_property("__proto__", get_model_prototype(env.clone(), teo_object.model().name().to_owned()))?;
    env.wrap(&mut js_object, teo_object)?;
    Ok(js_object)
}

#[napi(js_name = "App")]
pub struct App {
    builder: AppBuilder
}

/// A Teo app.
#[napi]
impl App {

    /// Create a Teo app.
    #[napi(constructor)]
    pub fn new(env: Env) -> Self {
        Self::with_cli(env, false)
    }

    /// @internal
    #[napi(factory)]
    pub fn with_cli(env: Env, cli: bool) -> Self {
        let entrance = if cli { Entrance::CLI } else { Entrance::APP };
        let global = env.get_global().unwrap();
        let process: JsObject = global.get_named_property("process").unwrap();
        let version: JsString = process.get_named_property("version").unwrap();
        let version_str: String = version.into_utf8().unwrap().as_str().unwrap().to_owned();
        App { builder: AppBuilder::new_with_environment_version_and_entrance(teo::core::app::environment::EnvironmentVersion::NodeJS(version_str), entrance) }
    }

    // /// Run this app.
    // #[napi]
    // pub fn run(&self, env: Env) {
    //     let
    //     let promise: Promise<JsUndefined> = ctx.env.execute_tokio_future((|| async {
    //         let mut_builder = self.builder.to_mut();
    //         mut_builder.build().await
    //     })(), |env, app: App| {
    //         env.get_undefined()
    //     })?;
    //     let teo_app = mut_builder.build().await;
    //     // self.generate_classes(&teo_app);
    //     let _ = teo_app.run().await;
    // }

    /// Run this app.
    #[napi]
    pub fn run(&self, env: Env) {
        let mut_builder = self.builder.to_mut();
        let teo_app = Box::leak(Box::new(tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(mut_builder.build())));
        self.generate_classes(teo_app, env).unwrap();

        let _ = tokio::spawn(teo_app.run());
    }

    fn generate_classes(&self, teo_app: &teo::core::app::App, env: Env) -> Result<()> {
        let graph = teo_app.graph();
        for model in graph.models() {
            let leaked_model_name = Box::leak(Box::new(model.name().to_owned()));
            let model_name = model.name();
            let ctor = get_model_class(env, model_name.to_owned());
            let mut ctor_object = ctor.coerce_to_object()?;
            let mut prototype: JsObject = ctor_object.get_named_property("prototype")?;
            // find unique
            let find_unique = env.create_function_from_closure("findUnique", |ctx| {
                let teo_value = if ctx.length == 0 {
                    TeoValue::HashMap(HashMap::new())
                } else {
                    let unknown: JsUnknown = ctx.get(0)?;
                    js_unknown_to_teo_value(unknown, ctx.env.clone())
                };
                let promise = ctx.env.execute_tokio_future((|| async {
                    let v = teo_value;
                    match Graph::current().find_unique(leaked_model_name, &v).await {
                        Ok(obj) => Ok(obj),
                        Err(err) => Err(Error::from_reason(err.message())),
                    }
                })(), |env, object: TeoObject| {
                    js_object_from_teo_object(env, object)
                })?;
                Ok(promise)
            })?;
            ctor_object.set_named_property("findUnique", find_unique)?;
            // find first
            let find_unique = env.create_function_from_closure("findFirst", |ctx| {
                let teo_value = if ctx.length == 0 {
                    TeoValue::HashMap(HashMap::new())
                } else {
                    let unknown: JsUnknown = ctx.get(0)?;
                    js_unknown_to_teo_value(unknown, ctx.env.clone())
                };
                let promise = ctx.env.execute_tokio_future((|| async {
                    let v = teo_value;
                    match Graph::current().find_first(leaked_model_name, &v).await {
                        Ok(obj) => Ok(obj),
                        Err(err) => Err(Error::from_reason(err.message())),
                    }
                })(), |env, object: TeoObject| {
                    js_object_from_teo_object(env, object)
                })?;
                Ok(promise)
            })?;
            ctor_object.set_named_property("findFirst", find_unique)?;
            // find many
            let find_many = env.create_function_from_closure("findMany", |ctx| {
                let teo_value = if ctx.length == 0 {
                    TeoValue::HashMap(HashMap::new())
                } else {
                    let unknown: JsUnknown = ctx.get(0)?;
                    js_unknown_to_teo_value(unknown, ctx.env.clone())
                };
                let promise = ctx.env.execute_tokio_future((|| async {
                    let v = teo_value;
                    match Graph::current().find_many(leaked_model_name, &v).await {
                        Ok(objects) => Ok(objects),
                        Err(err) => Err(Error::from_reason(err.message())),
                    }
                })(), |env, objects: Vec<TeoObject>| {
                    let mut array = env.create_array_with_length(objects.len())?;
                    for (index, object) in objects.iter().enumerate() {
                        array.set_element(index as u32, js_object_from_teo_object(env, object.clone())?)?;
                    }
                    Ok(array)
                })?;
                Ok(promise)
            })?;
            ctor_object.set_named_property("findMany", find_many)?;
            // create
            let create = env.create_function_from_closure("create", |ctx| {
                let teo_value = if ctx.length == 0 {
                    TeoValue::HashMap(HashMap::new())
                } else {
                    let unknown: JsUnknown = ctx.get(0)?;
                    js_unknown_to_teo_value(unknown, ctx.env.clone())
                };
                let promise = ctx.env.execute_tokio_future((|| async {
                    Ok(Graph::current().create_object(leaked_model_name, teo_value).await.unwrap())
                })(), |env, object: TeoObject| {
                    js_object_from_teo_object(env, object)
                })?;
                Ok(promise)
            })?;
            ctor_object.set_named_property("create", create)?;
            // isNew
            let is_new = env.create_function_from_closure("isNew", |ctx| {
                let this: JsObject = ctx.this()?;
                let object: &mut TeoObject = ctx.env.unwrap(&this)?;
                ctx.env.get_boolean(object.is_new())
            })?;
            prototype.set_named_property("isNew", is_new)?;
            // isModified
            let is_modified = env.create_function_from_closure("isModified", |ctx| {
                let this: JsObject = ctx.this()?;
                let object: &mut TeoObject = ctx.env.unwrap(&this)?;
                ctx.env.get_boolean(object.is_modified())
            })?;
            prototype.set_named_property("isModified", is_modified)?;
            // set
            let set = env.create_function_from_closure("set", |ctx| {
                let unknown: JsUnknown = ctx.get(0)?;
                let input = js_unknown_to_teo_value(unknown, ctx.env.clone());
                let this: JsObject = ctx.this()?;
                let object: &mut TeoObject = ctx.env.unwrap(&this)?;
                let object = object.clone();
                let promise = ctx.env.execute_tokio_future((|| async move {
                    let i = input;
                    match object.set_teon(&i).await {
                        Ok(()) => Ok(()),
                        Err(err) => Err(Error::from_reason(err.message())),
                    }
                })(), |&mut env, result: ()| {
                    env.get_undefined()
                })?;
                Ok(promise)
            })?;
            prototype.set_named_property("set", set)?;
            // update
            let update = env.create_function_from_closure("update", |ctx| {
                let unknown: JsUnknown = ctx.get(0)?;
                let input = js_unknown_to_teo_value(unknown, ctx.env.clone());
                let this: JsObject = ctx.this()?;
                let object: &mut TeoObject = ctx.env.unwrap(&this)?;
                let object = object.clone();
                let promise = ctx.env.execute_tokio_future((|| async move {
                    let i = input;
                    match object.update_teon(&i).await {
                        Ok(()) => Ok(()),
                        Err(err) => Err(Error::from_reason(err.message())),
                    }
                })(), |&mut env, result: ()| {
                    env.get_undefined()
                })?;
                Ok(promise)
            })?;
            prototype.set_named_property("update", update)?;
            // save
            let save = env.create_function_from_closure("save", |ctx| {
                let this: JsObject = ctx.this()?;
                let object: &mut TeoObject = ctx.env.unwrap(&this)?;
                let object = object.clone();
                let promise = ctx.env.execute_tokio_future((|| async move {
                    match object.save().await {
                        Ok(()) => Ok(()),
                        Err(err) => Err(Error::from_reason(err.message())),
                    }
                })(), |&mut env, result: ()| {
                    env.get_undefined()
                })?;
                Ok(promise)
            })?;
            prototype.set_named_property("save", save)?;
            // delete
            let delete = env.create_function_from_closure("delete", |ctx| {
                let this: JsObject = ctx.this()?;
                let object: &mut TeoObject = ctx.env.unwrap(&this)?;
                let object = object.clone();
                let promise = ctx.env.execute_tokio_future((|| async move {
                    match object.delete().await {
                        Ok(()) => Ok(()),
                        Err(err) => Err(Error::from_reason(err.message())),
                    }
                })(), |&mut env, result: ()| {
                    env.get_undefined()
                })?;
                Ok(promise)
            })?;
            prototype.set_named_property("delete", delete)?;
            // for field in model.fields() {

            // }
        }
        Ok(())
    }

    /// Register a named transformer.
    #[napi(ts_args_type = "callback: (input: any) => any | Promise<any>")]
    pub fn transform(&self, name: String, callback: JsFunction) -> Result<()> {
        let mut_builder = self.builder.to_mut();
        let tsfn: ThreadsafeFunction<TeoValue, ErrorStrategy::Fatal> = callback.create_threadsafe_function(0, |ctx| {
            let js_value = teo_value_to_js_unknown(&ctx.value, &ctx);
            Ok(vec![js_value])
        })?;
        let tsfn_cloned = Box::leak(Box::new(tsfn));
        mut_builder.transform(name, |value: TeoValue| async {
            let result: WrappedTeoValue = tsfn_cloned.call_async(value).await.unwrap();
            result.to_teo_value().await
        });
        Ok(())
    }

    /// Register a named validator.
    #[napi(ts_args_type = "callback: (input: any) => boolean | string | undefined | null | Promise<boolean | string | undefined | null>")]
    pub fn validate(&self, name: String, callback: JsFunction) -> Result<()> {
        let mut_builder = self.builder.to_mut();
        let tsfn: ThreadsafeFunction<TeoValue, ErrorStrategy::Fatal> = callback.create_threadsafe_function(0, |ctx| {
            let js_value = teo_value_to_js_unknown(&ctx.value, &ctx);
            Ok(vec![js_value])
        })?;
        let tsfn_cloned = Box::leak(Box::new(tsfn));
        mut_builder.validate(name, |value: TeoValue| async {
            let result: WrappedTeoValue = tsfn_cloned.call_async(value).await.unwrap();
            let teo_value = result.to_teo_value().await;
            match teo_value {
                TeoValue::String(s) => {
                    ValidateResult::Validity(Validity::Invalid(s.to_owned()))
                },
                TeoValue::Bool(b) => if b {
                    ValidateResult::Validity(Validity::Valid)
                } else {
                    ValidateResult::Validity(Validity::Invalid("value is invalid".to_owned()))
                },
                _ => ValidateResult::Validity(Validity::Valid)
            }
        });
        Ok(())
    }

    /// Register a named callback.
    #[napi(ts_args_type = "callback: (input: any) => void | Promise<void>")]
    pub fn callback(&self, name: String, callback: JsFunction) -> Result<()> {
        let mut_builder = self.builder.to_mut();
        let tsfn: ThreadsafeFunction<TeoValue, ErrorStrategy::Fatal> = callback.create_threadsafe_function(0, |ctx| {
            let js_value = teo_value_to_js_unknown(&ctx.value, &ctx);
            Ok(vec![js_value])
        })?;
        let tsfn_cloned = Box::leak(Box::new(tsfn));
        mut_builder.callback(name, |value: TeoValue| async {
            let result: WrappedTeoValue = tsfn_cloned.call_async(value).await.unwrap();
            let _teo_value = result.to_teo_value().await;
        });
        Ok(())
    }

    #[napi(js_name = "compare<T>", ts_args_type = "callback: (oldValue: T, newValue: T) => boolean | string | undefined | null | Promise<boolean | string | undefined | null>")]
    pub fn compare(&self, name: String, callback: JsFunction) -> Result<()> {
        let mut_builder = self.builder.to_mut();
        let tsfn: ThreadsafeFunction<(TeoValue, TeoValue), ErrorStrategy::Fatal> = callback.create_threadsafe_function(0, |ctx: ThreadSafeCallContext<(TeoValue, TeoValue)>| {
            let js_value_0 = teo_value_to_js_unknown(&ctx.value.0, &ctx);
            let js_value_1 = teo_value_to_js_unknown(&ctx.value.1, &ctx);
            Ok(vec![js_value_0, js_value_1])
        })?;
        let tsfn_cloned = Box::leak(Box::new(tsfn));
        mut_builder.compare(name, |old: TeoValue, new: TeoValue| async {
            let result: WrappedTeoValue = tsfn_cloned.call_async((old, new)).await.unwrap();
            let teo_value = result.to_teo_value().await;
            match teo_value {
                TeoValue::String(s) => {
                    ValidateResult::Validity(Validity::Invalid(s.to_owned()))
                },
                TeoValue::Bool(b) => if b {
                    ValidateResult::Validity(Validity::Valid)
                } else {
                    ValidateResult::Validity(Validity::Invalid("value is invalid".to_owned()))
                },
                _ => ValidateResult::Validity(Validity::Valid)
            }
        });
        Ok(())
    }
}

#[module_exports]
pub fn init(mut _exports: JsObject, _env: Env) -> Result<()> {
    unsafe { CLASSES = Some(Box::leak(Box::new(HashMap::new()))) };
    Ok(())
}
