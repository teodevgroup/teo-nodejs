
use std::collections::HashMap;
use inflector::Inflector;
use napi::threadsafe_function::{ThreadsafeFunction, ErrorStrategy, ThreadSafeCallContext};
use napi::{Env, JsObject, JsString, JsFunction, Result, JsUnknown, Error, JsSymbol, CallContext, Property, ValueType};
use teo::app::ctx::AppCtx;
use teo::app::entrance::Entrance;
use teo::app::program::Program;
use teo::app::cli::run_without_prepare::run_without_prepare;
use teo::core::callbacks::types::validate::{ValidateResult, Validity};
use teo::core::object::{Object as TeoObject};
use teo::core::teon::Value as TeoValue;
use teo::app::app::App as TeoApp;
use teo::prelude::{ModelCtx, UserCtx};
use value::{teo_value_to_js_unknown, WrappedTeoValue};
use crate::value::{js_unknown_to_teo_value, TeoUnused};

static mut CTXS: Option<&'static HashMap<String, napi::Ref<()>>> = None;
static mut CLASSES: Option<&'static HashMap<String, napi::Ref<()>>> = None;
static mut OBJECTS: Option<&'static HashMap<String, napi::Ref<()>>> = None;

fn classes_mut() -> &'static mut HashMap<String, napi::Ref<()>> {
    unsafe {
        let const_ptr = CLASSES.unwrap() as *const HashMap<String, napi::Ref<()>>;
        let mut_ptr = const_ptr as *mut HashMap<String, napi::Ref<()>>;
        &mut *mut_ptr
    }
}

fn objects_mut() -> &'static mut HashMap<String, napi::Ref<()>> {
    unsafe {
        let const_ptr = OBJECTS.unwrap() as *const HashMap<String, napi::Ref<()>>;
        let mut_ptr = const_ptr as *mut HashMap<String, napi::Ref<()>>;
        &mut *mut_ptr
    }
}

fn ctxs_mut() -> &'static mut HashMap<String, napi::Ref<()>> {
    unsafe {
        let const_ptr = CTXS.unwrap() as *const HashMap<String, napi::Ref<()>>;
        let mut_ptr = const_ptr as *mut HashMap<String, napi::Ref<()>>;
        &mut *mut_ptr
    }
}

fn ctx_constructor_function(env: Env) -> Result<JsFunction> {
    let ctor = env.create_function_from_closure("TeoCtx", |ctx| {
        ctx.env.get_undefined()
    })?;
    let mut prototype = env.create_object()?;
    prototype.set_named_property("__teo_ctx__", env.get_boolean(true)?)?;
    let mut ctor_object = ctor.coerce_to_object()?;
    ctor_object.set_named_property("prototype", prototype)?;
    let r = env.create_reference(ctor_object)?;
    ctxs_mut().insert("User".to_owned(), r);
    let ref_get = unsafe { CTXS.unwrap().get("User").unwrap() };
    let object: JsFunction = env.get_reference_value(ref_get)?;
    Ok(object)
}

fn model_class_constructor_function(env: Env, name: &str) -> Result<JsFunction> {
    let ctor = env.create_function_from_closure(&name, |ctx| {
        // let this = ctx.this_unchecked();
        ctx.env.get_undefined()
    })?;
    let mut prototype = env.create_object()?;
    prototype.set_named_property("__teo_class__", env.get_boolean(true)?)?;
    let mut ctor_object = ctor.coerce_to_object()?;
    ctor_object.set_named_property("prototype", prototype)?;
    let r = env.create_reference(ctor_object)?;
    classes_mut().insert(name.to_owned(), r);
    let ref_get = unsafe { CLASSES.unwrap().get(name).unwrap() };
    let object: JsFunction = env.get_reference_value(ref_get)?;
    Ok(object)
}

fn model_object_constructor_function(env: Env, name: &str) -> Result<JsFunction> {
    let ctor = env.create_function_from_closure(&name, |ctx| {
        // let this = ctx.this_unchecked();
        ctx.env.get_undefined()
    })?;
    let mut prototype = env.create_object()?;
    prototype.set_named_property("__teo_object__", env.get_boolean(true)?)?;
    let mut ctor_object = ctor.coerce_to_object()?;
    ctor_object.set_named_property("prototype", prototype)?;
    let r = env.create_reference(ctor_object)?;
    objects_mut().insert(name.to_owned(), r);
    let ref_get = unsafe { OBJECTS.unwrap().get(name).unwrap() };
    let object: JsFunction = env.get_reference_value(ref_get)?;
    Ok(object)
}

pub fn get_model_class_class(env: Env, name: &str) -> JsFunction {
    unsafe {
        if let Some(object_ref) = CLASSES.unwrap().get(name) {
            let object: JsFunction = env.get_reference_value(object_ref).unwrap();
            object
        } else {
            model_class_constructor_function(env, name).unwrap()
        }
    }
}

pub fn get_model_object_class(env: Env, name: &str) -> JsFunction {
    unsafe {
        if let Some(object_ref) = OBJECTS.unwrap().get(name) {
            let object: JsFunction = env.get_reference_value(object_ref).unwrap();
            object
        } else {
            model_object_constructor_function(env, name).unwrap()
        }
    }
}

pub fn get_user_ctx_class(env: Env) -> JsFunction {
    unsafe {
        if let Some(object_ref) = CTXS.unwrap().get("User") {
            let object: JsFunction = env.get_reference_value(object_ref).unwrap();
            object
        } else {
            ctx_constructor_function(env).unwrap()
        }
    }
}

fn get_model_object_prototype(env: Env, name: &str) -> JsObject {
    let js_function = get_model_object_class(env, name);
    let js_object = js_function.coerce_to_object().unwrap();
    let prototype: JsObject = js_object.get_named_property("prototype").unwrap();
    prototype
}

fn get_js_user_ctx_prototype(env: Env) -> JsObject {
    let js_function = get_user_ctx_class(env);
    let js_object = js_function.coerce_to_object().unwrap();
    let prototype: JsObject = js_object.get_named_property("prototype").unwrap();
    prototype
}

fn get_model_class_prototype(env: Env, name: &str) -> JsObject {
    let js_function = get_model_class_class(env, name);
    let js_object = js_function.coerce_to_object().unwrap();
    let prototype: JsObject = js_object.get_named_property("prototype").unwrap();
    prototype
}

pub(crate) fn js_model_ctx_from_teo_model_ctx(env: Env, model_ctx: ModelCtx, name: &str) -> Result<JsObject> {
    let mut js_object = env.create_object()?;
    js_object.set_named_property("__proto__", get_model_class_prototype(env.clone(), name))?;
    env.wrap(&mut js_object, model_ctx)?;
    Ok(js_object)
}

pub(crate) fn js_object_from_teo_object(env: Env, teo_object: TeoObject) -> Result<JsObject> {
    let mut js_object = env.create_object()?;
    js_object.set_named_property("__proto__", get_model_object_prototype(env.clone(), teo_object.model().name()))?;
    env.wrap(&mut js_object, teo_object)?;
    Ok(js_object)
}

pub(crate) fn js_optional_object_from_teo_object(env: Env, teo_object: Option<TeoObject>) -> Result<JsUnknown> {
    Ok(match teo_object {
        Some(teo_object) => js_object_from_teo_object(env, teo_object)?.into_unknown(),
        None => env.get_undefined()?.into_unknown(),
    })
}

pub(crate) fn js_user_ctx_from_user_ctx(env: Env, user_ctx: UserCtx) -> Result<JsObject> {
    let mut js_object = env.create_object()?;
    let prototype = get_js_user_ctx_prototype(env.clone());
    js_object.set_named_property("__proto__", prototype)?;
    env.wrap(&mut js_object, user_ctx)?;
    Ok(js_object)
}

#[module_exports]
pub fn init(mut _exports: JsObject, _env: Env) -> Result<()> {
    unsafe { CLASSES = Some(Box::leak(Box::new(HashMap::new()))) };
    unsafe { OBJECTS = Some(Box::leak(Box::new(HashMap::new()))) };
    unsafe { CTXS = Some(Box::leak(Box::new(HashMap::new()))) };
    Ok(())
}
