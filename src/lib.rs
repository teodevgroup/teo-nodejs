#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

pub mod value;

use std::collections::HashMap;
use std::sync::Mutex;
use napi::threadsafe_function::{ThreadsafeFunction, ErrorStrategy, ThreadSafeCallContext};
use napi::{Env, JsObject, JsString, JsFunction, Result, JsUndefined, CallContext, Property};
use teo::core::app::{builder::AppBuilder, entrance::Entrance};
use teo::core::pipeline::items::function::validate::{ValidateResult, Validity};
use teo::core::teon::Value as TeoValue;
use to_mut::ToMut;
use value::{teo_value_to_js_unknown, WrappedTeoValue};

static mut CLASSES: Option<&'static HashMap<String, JsObject>> = None;

fn classes_mut() -> &'static mut HashMap<String, JsObject> {
    unsafe {
        let const_ptr = CLASSES.unwrap() as *const HashMap<String, JsObject>;
        let mut_ptr = const_ptr as *mut HashMap<String, JsObject>;
        &mut *mut_ptr
    }
}

#[js_function(1)]
fn model_class_constructor(ctx: CallContext) -> Result<JsUndefined> {
    ctx.env.get_undefined()
}

//env.create_function_from_closure(name, callback)

#[napi]
fn get_model_class(name: String) -> &'static JsObject {
    unsafe { CLASSES.unwrap().get(name.as_str()).unwrap() }
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

    /// Run this app.
    #[napi]
    pub async fn run(&self) {
        let mut_builder = self.builder.to_mut();
        let teo_app = mut_builder.build().await;
        // self.generate_classes(&teo_app);
        let _ = teo_app.run().await;
    }

    fn generate_classes(&self, teo_app: &teo::core::app::App, env: Env) -> Result<()> {
        unsafe { CLASSES = Some(Box::leak(Box::new(HashMap::new()))) };
        let graph = teo_app.graph();
        for model in graph.models() {
            let model_name = Box::leak(Box::new(model.name().to_owned()));
            // constructor
            let ctor = env.create_function_from_closure(model.name(), |ctx| {
                // let this = ctx.this_unchecked();
                ctx.env.get_undefined()
            })?;

            let mut prototype = env.create_object()?;
            // create
            let create = env.create_function_from_closure("create", |ctx| {
                let mut object = ctx.env.create_object()?;
                let _ = object.set_named_property("__proto__", unsafe { CLASSES.unwrap().get(model_name).unwrap() });
                Ok(object)
            })?;
            prototype.set_named_property("create", create)?;
            let mut ctor_obj = ctor.coerce_to_object().unwrap();
            ctor_obj.set_named_property("prototype", prototype)?;
            for field in model.fields() {

            }
            classes_mut().insert(model.name().to_owned(), ctor_obj);
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
pub fn init(mut exports: JsObject, env: Env) -> Result<()> {
    let _ = exports.set_named_property("WTF", env.create_int32(45).unwrap());
    let ctor = env.create_function_from_closure("User", |ctx| {
        Ok(ctx.env.get_undefined().unwrap())
    }).unwrap();
    let mut prototype = env.create_object().unwrap();
    let _ = prototype.set_named_property("a", env.create_int32(5).unwrap());
    let mut ctor_obj = ctor.coerce_to_object().unwrap();
    let _ = ctor_obj.set_named_property("prototype", prototype);
    let _ = exports.set_named_property("User", ctor_obj);
    Ok(())
}
