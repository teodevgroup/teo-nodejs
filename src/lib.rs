#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

pub mod value;

use std::collections::HashMap;
use inflector::Inflector;
use napi::threadsafe_function::{ThreadsafeFunction, ErrorStrategy, ThreadSafeCallContext};
use napi::{Env, JsObject, JsString, JsFunction, Result, JsUnknown, Error, JsSymbol, CallContext, Property, ValueType};
use teo::app::ctx::AppCtx;
use teo::app::entrance::Entrance;
use teo::app::program::Program;
use teo::core::callbacks::types::validate::{ValidateResult, Validity};
use teo::core::object::{Object as TeoObject};
use teo::core::teon::Value as TeoValue;
use teo::app::app::App as TeoApp;
use teo::prelude::{ModelCtx, UserCtx};
use value::{teo_value_to_js_unknown, WrappedTeoValue};
use crate::value::{js_unknown_to_teo_value, TeoUnused};

pub trait IntoNodeJSResult<T> {
    fn into_nodejs_result(self) -> Result<T>;
}

impl<T> IntoNodeJSResult<T> for teo::prelude::Result<T> {
    fn into_nodejs_result(self) -> Result<T> {
        match self {
            Ok(r) => Ok(r),
            Err(e) => Err(Error::from_reason(e.message())),
        }
    }
}

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
    classes_mut().insert(name.to_owned(), r);
    let ref_get = unsafe { CLASSES.unwrap().get(name).unwrap() };
    let object: JsFunction = env.get_reference_value(ref_get)?;
    Ok(object)
}

pub fn get_model_class_class(env: Env, name: &str) -> JsFunction {
    unsafe {
        if let Some(object_ref) = CLASSES.unwrap().get(name) {
            let object: JsFunction = env.get_reference_value(object_ref).unwrap();
            object
        } else {
            model_object_constructor_function(env, name).unwrap()
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

#[napi(js_name = "App")]
pub struct App {
    teo_app: TeoApp,
}

/// A Teo app.
#[napi]
impl App {

    /// Create a Teo app.
    #[napi(constructor)]
    pub fn new(env: Env) -> Result<Self> {
        Self::with_cli(env, false)
    }

    /// @internal
    #[napi(factory)]
    pub fn with_cli(env: Env, cli: bool) -> Result<Self> {
        let entrance = if cli { Entrance::CLI } else { Entrance::APP };
        let global = env.get_global()?;
        let process: JsObject = global.get_named_property("process")?;
        let version: JsString = process.get_named_property("version")?;
        let version_str: String = version.into_utf8()?.as_str()?.to_owned();
        let app_ctx = AppCtx::get().unwrap();
        let app = App { teo_app: TeoApp::new().unwrap() };
        app_ctx.set_entrance(entrance).into_nodejs_result()?;
        app_ctx.set_program(Program::NodeJS(version_str)).into_nodejs_result()?;
        Ok(app)
    }

    /// Run this app.
    #[napi(ts_return_type="Promise<void>")]
    pub fn run(&self, env: Env) -> Result<JsUnknown> {
        self.teo_app.prepare().into_nodejs_result()?;
        self.generate_classes(env)?;
        let js_function = env.create_function_from_closure("run", |ctx| {
            let promise = ctx.env.execute_tokio_future((|| async {
                let _ = self.teo_app.run().await;
                Ok(0)
            })(), |env: &mut Env, _unknown: i32| {
                env.get_undefined()
            })?;
            Ok(promise)
        })?;
        let result: JsUnknown = js_function.call(None, &[env.get_undefined()?])?;
        Ok(result)
    }

    fn generate_classes(&self, env: Env) -> Result<()> {
        let ctx_ctor = ctx_constructor_function(env)?;
        let ctx_ctor_object = ctx_ctor.coerce_to_object()?;
        let mut ctx_prototype: JsObject = ctx_ctor_object.get_named_property("prototype")?;
        let graph = AppCtx::get().into_nodejs_result()?.graph().into_nodejs_result()?;
        for model in graph.models() {
            let leaked_model_name = model.name();
            let ctx_property = Property::new(leaked_model_name)?.with_getter_closure(|env: Env, this: JsObject| {
                let user_ctx: &mut UserCtx = env.unwrap(&this)?;
                let model_ctx = user_ctx.model_ctx(leaked_model_name).into_nodejs_result()?;
                js_model_ctx_from_teo_model_ctx(env, model_ctx, leaked_model_name)
            });
            ctx_prototype.define_properties(&[ctx_property])?;
            let class_ctor = get_model_class_class(env, leaked_model_name);
            let class_ctor_object = class_ctor.coerce_to_object()?;
            let mut class_prototype: JsObject = class_ctor_object.get_named_property("prototype")?;
            let object_ctor = get_model_object_class(env, leaked_model_name);
            let object_ctor_object = object_ctor.coerce_to_object()?;
            let mut object_prototype: JsObject = object_ctor_object.get_named_property("prototype")?;
            // find unique
            let find_unique = env.create_function_from_closure("findUnique", |ctx| {
                let teo_value = if ctx.length == 0 {
                    TeoValue::HashMap(HashMap::new())
                } else {
                    let unknown: JsUnknown = ctx.get(0)?;
                    js_unknown_to_teo_value(unknown, ctx.env.clone())
                };
                let this: JsObject = ctx.this()?;
                let model_ctx: &mut ModelCtx = ctx.env.unwrap(&this)?;
                let model_ctx_clone = model_ctx.clone();
                let promise = ctx.env.execute_tokio_future((|| async move {
                    match model_ctx_clone.find_unique(&teo_value).await {
                        Ok(obj) => Ok(obj),
                        Err(err) => Err(Error::from_reason(err.message())),
                    }
                })(), |env, object: Option<TeoObject>| {
                    js_optional_object_from_teo_object(env.clone(), object)
                })?;
                Ok(promise)
            })?;
            class_prototype.set_named_property("findUnique", find_unique)?;
            // find first
            let find_unique = env.create_function_from_closure("findFirst", |ctx| {
                let teo_value = if ctx.length == 0 {
                    TeoValue::HashMap(HashMap::new())
                } else {
                    let unknown: JsUnknown = ctx.get(0)?;
                    js_unknown_to_teo_value(unknown, ctx.env.clone())
                };
                let this: JsObject = ctx.this()?;
                let model_ctx: &mut ModelCtx = ctx.env.unwrap(&this)?;
                let model_ctx_cloned = model_ctx.clone();
                let promise = ctx.env.execute_tokio_future((|| async move {
                    let v = teo_value;
                    match model_ctx_cloned.find_first(&v).await {
                        Ok(obj) => Ok(obj),
                        Err(err) => Err(Error::from_reason(err.message())),
                    }
                })(), |env, object: Option<TeoObject>| {
                    js_optional_object_from_teo_object(env.clone(), object)
                })?;
                Ok(promise)
            })?;
            class_prototype.set_named_property("findFirst", find_unique)?;
            // find many
            let find_many = env.create_function_from_closure("findMany", |ctx| {
                let teo_value = if ctx.length == 0 {
                    TeoValue::HashMap(HashMap::new())
                } else {
                    let unknown: JsUnknown = ctx.get(0)?;
                    js_unknown_to_teo_value(unknown, ctx.env.clone())
                };
                let this: JsObject = ctx.this()?;
                let model_ctx: &mut ModelCtx = ctx.env.unwrap(&this)?;
                let model_ctx_cloned = model_ctx.clone();
                let promise = ctx.env.execute_tokio_future((|| async move {
                    let v = teo_value;
                    match model_ctx_cloned.find_many(&v).await {
                        Ok(objects) => Ok(objects),
                        Err(err) => Err(Error::from_reason(err.message())),
                    }
                })(), |env, objects: Vec<TeoObject>| {
                    let mut array = env.create_array_with_length(objects.len())?;
                    for (index, object) in objects.iter().enumerate() {
                        array.set_element(index as u32, js_object_from_teo_object(env.clone(), object.clone())?)?;
                    }
                    Ok(array)
                })?;
                Ok(promise)
            })?;
            class_prototype.set_named_property("findMany", find_many)?;
            // create
            let create = env.create_function_from_closure("create", |ctx| {
                let teo_value = if ctx.length == 0 {
                    TeoValue::HashMap(HashMap::new())
                } else {
                    let unknown: JsUnknown = ctx.get(0)?;
                    js_unknown_to_teo_value(unknown, ctx.env.clone())
                };
                let this: JsObject = ctx.this()?;
                let model_ctx: &mut ModelCtx = ctx.env.unwrap(&this)?;
                let model_ctx_cloned = model_ctx.clone();
                let promise = ctx.env.execute_tokio_future((|| async move {
                    Ok(model_ctx_cloned.create_object(&teo_value).await.unwrap())
                })(), |env, object: TeoObject| {
                    js_object_from_teo_object(env.clone(), object)
                })?;
                Ok(promise)
            })?;
            class_prototype.set_named_property("create", create)?;
            // isNew
            let is_new = Property::new("isNew")?.with_getter_closure(|env: Env, this: JsObject| {
                let object: &mut TeoObject = env.unwrap(&this)?;
                env.get_boolean(object.is_new())
            });
            object_prototype.define_properties(&[is_new])?;
            // isModified
            let is_modified = Property::new("isModified")?.with_getter_closure(|env: Env, this: JsObject| {
                let object: &mut TeoObject = env.unwrap(&this)?;
                env.get_boolean(object.is_modified())
            });
            object_prototype.define_properties(&[is_modified])?;
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
                })(), |&mut env, _result: ()| {
                    env.get_undefined()
                })?;
                Ok(promise)
            })?;
            object_prototype.set_named_property("set", set)?;
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
                })(), |&mut env, _result: ()| {
                    env.get_undefined()
                })?;
                Ok(promise)
            })?;
            object_prototype.set_named_property("update", update)?;
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
                })(), |&mut env, _result: ()| {
                    env.get_undefined()
                })?;
                Ok(promise)
            })?;
            object_prototype.set_named_property("save", save)?;
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
                })(), |&mut env, _result: ()| {
                    env.get_undefined()
                })?;
                Ok(promise)
            })?;
            object_prototype.set_named_property("delete", delete)?;
            let inspect_func = env.create_function_from_closure("toString", |ctx| {
                let this: JsObject = ctx.this()?;
                let object: &mut TeoObject = ctx.env.unwrap(&this)?;
                let result = format!("{:?}", object);
                ctx.env.create_string(&result)
            })?;
            let require: JsFunction = env.get_global()?.get_named_property("require")?;
            let util = require.call(None, &[env.create_string("node:util").unwrap().into_unknown()])?.coerce_to_object()?;
            let inspect: JsObject = util.get_named_property("inspect")?;
            let custom: JsSymbol = inspect.get_named_property("custom")?;
            object_prototype.set_property(custom, inspect_func)?;
            let to_string = env.create_function_from_closure("toString", |ctx| {
                let this: JsObject = ctx.this()?;
                let object: &mut TeoObject = ctx.env.unwrap(&this)?;
                let result = format!("{:?}", object);
                ctx.env.create_string(&result)
            })?;
            object_prototype.set_named_property("toString", to_string)?;
            for field in model.fields() {
                let field_name = Box::leak(Box::new(field.name().to_owned()));
                let property = Property::new(field_name.as_str())?.with_getter_closure(|env: Env, this: JsObject| {
                    let object: &mut TeoObject = env.unwrap(&this)?;
                    let value: TeoValue = object.get(field_name.as_str()).unwrap();
                    Ok(teo_value_to_js_unknown(&value, &env))
                }).with_setter_closure(|env: Env, this: JsObject, arg0: JsUnknown| {
                    let teo_value = js_unknown_to_teo_value(arg0, env.clone());
                    let object: &mut TeoObject = env.unwrap(&this)?;
                    object.set(field_name.as_str(), teo_value).unwrap();
                    Ok(())
                });
                object_prototype.define_properties(&[property])?;
            }
            for relation in model.relations() {
                let name: &'static str = unsafe {
                    let s = relation.name();
                    let u = { s as *const str };
                    let v = &*u;
                    v
                };
                if relation.is_vec() {
                    // get
                    let get_relation = env.create_function_from_closure(&name, move |ctx: CallContext<'_>| {
                        let teo_value = if ctx.length == 0 {
                            TeoValue::HashMap(HashMap::new())
                        } else {
                            let val: JsUnknown = ctx.get(0)?;
                            js_unknown_to_teo_value(val, ctx.env.clone())
                        };
                        let this: JsObject = ctx.this()?;
                        let object: &mut TeoObject = ctx.env.unwrap(&this)?;
                        let object_cloned = object.clone();
                        let promise = ctx.env.execute_tokio_future((|| async move {
                            match object_cloned.force_get_relation_objects(name, teo_value).await {
                                Ok(objects) => Ok(objects),
                                Err(err) => Err(Error::from_reason(err.message())),
                            }
                        })(), |env, objects| {
                            let mut array = env.create_array_with_length(objects.len())?;
                            for (index, object) in objects.iter().enumerate() {
                                array.set_element(index as u32, js_object_from_teo_object(env.clone(), object.clone())?)?;
                            }
                            Ok(array)
                        })?;
                        Ok(promise)
                    })?;
                    object_prototype.set_named_property(&name, get_relation)?;
                    // set
                    let set_name = "set".to_owned() + &name.to_pascal_case();
                    let set_relation = env.create_function_from_closure(&name, move |ctx: CallContext<'_>| {
                        let array: JsObject = ctx.get(0)?;
                        let mut objects = vec![];
                        for i in 0..array.get_array_length()? {
                            let element: JsObject = array.get_element(i)?;
                            let obj: &mut TeoObject = ctx.env.unwrap(&element)?;
                            objects.push(obj.clone());
                        }
                        let this: JsObject = ctx.this()?;
                        let object: &mut TeoObject = ctx.env.unwrap(&this)?;
                        let object_cloned = object.clone();
                        let promise = ctx.env.execute_tokio_future((|| async move {
                            Ok(object_cloned.force_set_relation_objects(name, objects).await)
                        })(), |env, _objects| {
                            env.get_undefined()
                        })?;
                        Ok(promise)
                    })?;
                    object_prototype.set_named_property(&set_name, set_relation)?;
                    // add
                    let add_name = "addTo".to_owned() + &name.to_pascal_case();
                    let add_relation = env.create_function_from_closure(&name, move |ctx: CallContext<'_>| {
                        let array: JsObject = ctx.get(0)?;
                        let mut objects = vec![];
                        for i in 0..array.get_array_length()? {
                            let element: JsObject = array.get_element(i)?;
                            let obj: &mut TeoObject = ctx.env.unwrap(&element)?;
                            objects.push(obj.clone());
                        }
                        let this: JsObject = ctx.this()?;
                        let object: &mut TeoObject = ctx.env.unwrap(&this)?;
                        let object_cloned = object.clone();
                        let promise = ctx.env.execute_tokio_future((|| async move {
                            Ok(object_cloned.force_add_relation_objects(name, objects).await)
                        })(), |env, _objects| {
                            env.get_undefined()
                        })?;
                        Ok(promise)
                    })?;
                    object_prototype.set_named_property(&add_name, add_relation)?;
                    // remove
                    let remove_name = "removeFrom".to_owned() + &name.to_pascal_case();
                    let remove_relation = env.create_function_from_closure(&name, move |ctx: CallContext<'_>| {
                        let array: JsObject = ctx.get(0)?;
                        let mut objects = vec![];
                        for i in 0..array.get_array_length()? {
                            let element: JsObject = array.get_element(i)?;
                            let obj: &mut TeoObject = ctx.env.unwrap(&element)?;
                            objects.push(obj.clone());
                        }
                        let this: JsObject = ctx.this()?;
                        let object: &mut TeoObject = ctx.env.unwrap(&this)?;
                        let object_cloned = object.clone();
                        let promise = ctx.env.execute_tokio_future((|| async move {
                            Ok(object_cloned.force_add_relation_objects(name, objects).await)
                        })(), |env, _objects| {
                            env.get_undefined()
                        })?;
                        Ok(promise)
                    })?;
                    object_prototype.set_named_property(&remove_name, remove_relation)?;
                } else {
                    // get
                    let mut property = Property::new(name)?;
                    property = property.with_getter_closure(move |env: Env, this: JsObject| {
                        let object: &mut TeoObject = env.unwrap(&this)?;
                        let object_cloned = object.clone();
                        let promise = env.execute_tokio_future((|| async move {
                            match object_cloned.force_get_relation_object(name).await {
                                Ok(v) => Ok(v),
                                Err(err) => Err(Error::from_reason(err.message())),
                            }
                        })(), |env, v: Option<TeoObject>| {
                            match v {
                                Some(obj) => Ok(js_object_from_teo_object(env.clone(), obj)?.into_unknown()),
                                None => Ok(env.get_undefined()?.into_unknown()),
                            }
                        })?;
                        Ok(promise)
                    });
                    object_prototype.define_properties(&[property])?;
                    // set
                    let set_name = "set".to_owned() + &name.to_pascal_case();
                    let set_relation = env.create_function_from_closure(&name, move |ctx: CallContext<'_>| {
                        let value: JsUnknown = ctx.get(0)?;
                        let arg = match value.get_type()? {
                            ValueType::Null | ValueType::Undefined => None,
                            ValueType::Object => {
                                let object = value.coerce_to_object()?;
                                let obj: &mut TeoObject = ctx.env.unwrap(&object)?;
                                Some(obj.clone())
                            }
                            _ => None,
                        };
                        let this: JsObject = ctx.this()?;
                        let object: &mut TeoObject = ctx.env.unwrap(&this)?;
                        let object_cloned = object.clone();
                        let promise = ctx.env.execute_tokio_future((|| async move {
                            Ok(object_cloned.force_set_relation_object(name, arg).await)
                        })(), |env, _objects| {
                            env.get_undefined()
                        })?;
                        Ok(promise)
                    })?;
                    object_prototype.set_named_property(&set_name, set_relation)?;
                }
            }
            for model_property in model.properties() {
                let field_name: &'static str = unsafe {
                    let s = model_property.name();
                    let u = { s as *const str };
                    let v = &*u;
                    v
                };
                if model_property.has_setter() {
                    let name = "set".to_owned() + &field_name.to_pascal_case();
                    let set_property = env.create_function_from_closure(&name, move |ctx: CallContext<'_>| {
                        let val: JsUnknown = ctx.get(0)?;
                        let teo_value = js_unknown_to_teo_value(val, ctx.env.clone());
                        let this: JsObject = ctx.this()?;
                        let object: &mut TeoObject = ctx.env.unwrap(&this)?;
                        let object_cloned = object.clone();
                        let promise = ctx.env.execute_tokio_future((|| async move {
                            match object_cloned.set_property(field_name, teo_value).await {
                                Ok(()) => Ok(()),
                                Err(err) => Err(Error::from_reason(err.message())),
                            }
                        })(), |_env, v: ()| {
                            Ok(v)
                        })?;
                        Ok(promise)
                    })?;
                    object_prototype.set_named_property(&name, set_property)?;
                }
                if model_property.has_getter() {
                    let mut property = Property::new(field_name)?;
                    property = property.with_getter_closure(move |env: Env, this: JsObject| {
                        let object: &mut TeoObject = env.unwrap(&this)?;
                        let object_cloned = object.clone();
                        let promise = env.execute_tokio_future((|| async move {
                            match object_cloned.get_property::<TeoValue>(field_name).await {
                                Ok(v) => Ok(v),
                                Err(err) => Err(Error::from_reason(err.message())),
                            }
                        })(), |env, v: TeoValue| {
                            Ok(teo_value_to_js_unknown(&v, env))
                        })?;
                        Ok(promise)
                    });
                    object_prototype.define_properties(&[property])?;
                }
            }
        }
        Ok(())
    }

    /// Register a named transformer.
    #[napi(ts_args_type = "name: string, callback: (input: any) => any | Promise<any>")]
    pub fn transform(&self, name: String, callback: JsFunction) -> Result<()> {
        let tsfn: ThreadsafeFunction<(TeoValue, TeoObject, UserCtx), ErrorStrategy::Fatal> = callback.create_threadsafe_function(0, |ctx: ThreadSafeCallContext<(TeoValue, TeoObject, UserCtx)>| {
            let js_value = teo_value_to_js_unknown(&ctx.value.0, &ctx.env);
            let js_value_object = js_value.coerce_to_object()?;
            let js_object = js_object_from_teo_object(ctx.env, ctx.value.1.clone())?;
            let js_ctx = js_user_ctx_from_user_ctx(ctx.env, ctx.value.2.clone())?;
            Ok(vec![js_value_object, js_object, js_ctx])
        })?;
        let tsfn_cloned = Box::leak(Box::new(tsfn));
        self.teo_app.transform(Box::leak(Box::new(name)).as_str(), |value: TeoValue, object: TeoObject, ctx: UserCtx| async {
            let result: WrappedTeoValue = tsfn_cloned.call_async((value, object, ctx)).await.unwrap();
            result.to_teo_value().await
        }).into_nodejs_result()?;
        Ok(())
    }

    /// Register a named validator.
    #[napi(ts_args_type = "name: string, callback: (input: any) => boolean | string | undefined | null | Promise<boolean | string | undefined | null>")]
    pub fn validate(&self, name: String, callback: JsFunction) -> Result<()> {
        let tsfn: ThreadsafeFunction<(TeoValue, TeoObject, UserCtx), ErrorStrategy::Fatal> = callback.create_threadsafe_function(0, |ctx: ThreadSafeCallContext<(TeoValue, TeoObject, UserCtx)>| {
            let js_value = teo_value_to_js_unknown(&ctx.value.0, &ctx.env);
            let js_value_object = js_value.coerce_to_object()?;
            let js_object = js_object_from_teo_object(ctx.env, ctx.value.1.clone())?;
            let js_ctx = js_user_ctx_from_user_ctx(ctx.env, ctx.value.2.clone())?;
            Ok(vec![js_value_object, js_object, js_ctx])
        })?;
        let tsfn_cloned = Box::leak(Box::new(tsfn));
        self.teo_app.validate(Box::leak(Box::new(name)).as_str(), |value: TeoValue, object: TeoObject, ctx: UserCtx| async {
            let result: WrappedTeoValue = tsfn_cloned.call_async((value, object, ctx)).await.unwrap();
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
        }).into_nodejs_result()?;
        Ok(())
    }

    /// Register a named callback.
    #[napi(ts_args_type = "name: string, callback: (input: any) => void | Promise<void>")]
    pub fn callback(&self, name: String, callback: JsFunction) -> Result<()> {
        let tsfn: ThreadsafeFunction<(TeoValue, TeoObject, UserCtx), ErrorStrategy::Fatal> = callback.create_threadsafe_function(0, |ctx: ThreadSafeCallContext<(TeoValue, TeoObject, UserCtx)>| {
            let js_value = teo_value_to_js_unknown(&ctx.value.0, &ctx.env);
            let js_value_object = js_value.coerce_to_object()?;
            let js_object = js_object_from_teo_object(ctx.env, ctx.value.1.clone())?;
            let js_ctx = js_user_ctx_from_user_ctx(ctx.env, ctx.value.2.clone())?;
            Ok(vec![js_value_object, js_object, js_ctx])
        })?;
        let tsfn_cloned = Box::leak(Box::new(tsfn));
        self.teo_app.callback(Box::leak(Box::new(name)).as_str(), |value: TeoValue, object: TeoObject, ctx: UserCtx| async {
            let result: WrappedTeoValue = tsfn_cloned.call_async((value, object, ctx)).await.unwrap();
            let _teo_value = result.to_teo_value().await;
        }).into_nodejs_result()?;
        Ok(())
    }

    #[napi(js_name = "compare<T>", ts_args_type = "name: string, callback: (oldValue: T, newValue: T) => boolean | string | undefined | null | Promise<boolean | string | undefined | null>")]
    pub fn compare(&self, name: String, callback: JsFunction) -> Result<()> {
        let tsfn: ThreadsafeFunction<(TeoValue, TeoValue, TeoObject, UserCtx), ErrorStrategy::Fatal> = callback.create_threadsafe_function(0, |ctx: ThreadSafeCallContext<(TeoValue, TeoValue, TeoObject, UserCtx)>| {
            let js_value_old = teo_value_to_js_unknown(&ctx.value.0, &ctx.env);
            let js_value_new = teo_value_to_js_unknown(&ctx.value.1, &ctx.env);
            let js_object = js_object_from_teo_object(ctx.env, ctx.value.2.clone())?.into_unknown();
            let js_ctx = js_user_ctx_from_user_ctx(ctx.env, ctx.value.3.clone())?.into_unknown();
            Ok(vec![js_value_old, js_value_new, js_object, js_ctx])
        })?;
        let tsfn_cloned = Box::leak(Box::new(tsfn));
        self.teo_app.compare(Box::leak(Box::new(name)).as_str(), |old: TeoValue, new: TeoValue, object: TeoObject, ctx: UserCtx| async {
            let result: WrappedTeoValue = tsfn_cloned.call_async((old, new, object, ctx)).await.unwrap();
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
        }).into_nodejs_result()?;
        Ok(())
    }

    /// Run before server is started.
    #[napi(ts_args_type = "callback: () => void | Promise<void>")]
    pub fn setup(&self, callback: JsFunction) -> Result<()> {
        let tsfn: ThreadsafeFunction<i32, ErrorStrategy::Fatal> = callback.create_threadsafe_function(0, |ctx| {
            let undefined = ctx.env.get_undefined()?;
            Ok(vec![undefined])
        })?;
        let tsfn_cloned = Box::leak(Box::new(tsfn));
        self.teo_app.setup(|| async {
            let _: Result<TeoUnused> = tsfn_cloned.call_async(0).await;
            Ok(())
        }).into_nodejs_result()?;
        Ok(())
    }
}

#[module_exports]
pub fn init(mut _exports: JsObject, _env: Env) -> Result<()> {
    unsafe { CLASSES = Some(Box::leak(Box::new(HashMap::new()))) };
    unsafe { OBJECTS = Some(Box::leak(Box::new(HashMap::new()))) };
    unsafe { CTXS = Some(Box::leak(Box::new(HashMap::new()))) };
    Ok(())
}
