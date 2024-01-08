use indexmap::IndexMap;
use napi::{Result, Error, Env, JsObject, JsFunction, JsUnknown, Property, JsSymbol, CallContext, ValueType, JsString, threadsafe_function::{ThreadSafeCallContext, ThreadsafeFunction, ErrorStrategy}};
use teo::prelude::{App, model, transaction, Namespace, object::Object as TeoObject, Value as TeoValue};
use std::collections::BTreeMap;
use inflector::Inflector;

use crate::{object::{js_any_to_teo_object, teo_object_to_js_any, value::teo_value_to_js_any}, result::{IntoNodeJSResult, IntoTeoResult, IntoTeoPathResult}};

static mut CTXS: Option<&'static BTreeMap<String, napi::Ref<()>>> = None;
static mut CLASSES: Option<&'static BTreeMap<String, napi::Ref<()>>> = None;
static mut OBJECTS: Option<&'static BTreeMap<String, napi::Ref<()>>> = None;

#[module_exports]
pub fn init(mut _exports: JsObject, _env: Env) -> Result<()> {
    unsafe { CLASSES = Some(Box::leak(Box::new(BTreeMap::new()))) };
    unsafe { OBJECTS = Some(Box::leak(Box::new(BTreeMap::new()))) };
    unsafe { CTXS = Some(Box::leak(Box::new(BTreeMap::new()))) };
    
    Ok(())
}

fn classes_mut() -> &'static mut BTreeMap<String, napi::Ref<()>> {
    unsafe {
        let const_ptr = CLASSES.unwrap() as *const BTreeMap<String, napi::Ref<()>>;
        let mut_ptr = const_ptr as *mut BTreeMap<String, napi::Ref<()>>;
        &mut *mut_ptr
    }
}

fn objects_mut() -> &'static mut BTreeMap<String, napi::Ref<()>> {
    unsafe {
        let const_ptr = OBJECTS.unwrap() as *const BTreeMap<String, napi::Ref<()>>;
        let mut_ptr = const_ptr as *mut BTreeMap<String, napi::Ref<()>>;
        &mut *mut_ptr
    }
}

fn ctxs_mut() -> &'static mut BTreeMap<String, napi::Ref<()>> {
    unsafe {
        let const_ptr = CTXS.unwrap() as *const BTreeMap<String, napi::Ref<()>>;
        let mut_ptr = const_ptr as *mut BTreeMap<String, napi::Ref<()>>;
        &mut *mut_ptr
    }
}

pub fn get_model_class_constructor(env: Env, name: &str) -> JsFunction {
    unsafe {
        if let Some(object_ref) = CLASSES.unwrap().get(name) {
            let object: JsFunction = env.get_reference_value(object_ref).unwrap();
            object
        } else {
            model_class_constructor_function(env, name).unwrap()
        }
    }
}

pub fn get_model_object_constructor(env: Env, name: &str) -> JsFunction {
    unsafe {
        if let Some(object_ref) = OBJECTS.unwrap().get(name) {
            let object: JsFunction = env.get_reference_value(object_ref).unwrap();
            object
        } else {
            model_object_constructor_function(env, name).unwrap()
        }
    }
}

pub fn get_ctx_constructor(env: Env, name: &str) -> JsFunction {
    unsafe {
        if let Some(object_ref) = CTXS.unwrap().get(name) {
            let object: JsFunction = env.get_reference_value(object_ref).unwrap();
            object
        } else {
            ctx_constructor_function(env, name).unwrap()
        }
    }
}

fn get_model_object_prototype(env: Env, name: &str) -> JsObject {
    let js_function = get_model_object_constructor(env, name);
    let js_object = js_function.coerce_to_object().unwrap();
    let prototype: JsObject = js_object.get_named_property("prototype").unwrap();
    prototype
}

fn get_model_class_prototype(env: Env, name: &str) -> JsObject {
    let js_function = get_model_class_constructor(env, name);
    let js_object = js_function.coerce_to_object().unwrap();
    let prototype: JsObject = js_object.get_named_property("prototype").unwrap();
    prototype
}

fn get_ctx_prototype(env: Env, name: &str) -> JsObject {
    let js_function = get_ctx_constructor(env, name);
    let js_object = js_function.coerce_to_object().unwrap();
    let prototype: JsObject = js_object.get_named_property("prototype").unwrap();
    prototype
}

fn ctx_constructor_function(env: Env, path: &str) -> Result<JsFunction> {
    let ctor = env.create_function_from_closure((path.to_owned() + "Ctx").as_str(), |ctx| {
        ctx.env.get_undefined()
    })?;
    let mut prototype = env.create_object()?;
    prototype.set_named_property("__teo_ctx__", env.get_boolean(true)?)?;
    let mut ctor_object = ctor.coerce_to_object()?;
    ctor_object.set_named_property("prototype", prototype)?;
    let r = env.create_reference(ctor_object)?;
    ctxs_mut().insert(path.to_owned(), r);
    let ref_get = unsafe { CTXS.unwrap().get(path).unwrap() };
    let object: JsFunction = env.get_reference_value(ref_get)?;
    Ok(object)
}

fn model_class_constructor_function(env: Env, name: &str) -> Result<JsFunction> {
    let ctor = env.create_function_from_closure(name, |ctx| {
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

pub(crate) fn js_model_class_object_from_teo_model_ctx(env: Env, model_ctx: model::Ctx, name: &str) -> Result<JsObject> {
    let mut js_object = env.create_object()?;
    js_object.set_named_property("__proto__", get_model_class_prototype(env.clone(), name))?;
    env.wrap(&mut js_object, model_ctx)?;
    Ok(js_object)
}

pub(crate) fn js_model_object_from_teo_model_object(env: Env, teo_model_object: model::Object) -> Result<JsObject> {
    let mut js_object = env.create_object()?;
    js_object.set_named_property("__proto__", get_model_object_prototype(env.clone(), &teo_model_object.model().path().join(".")))?;
    env.wrap(&mut js_object, teo_model_object)?;
    Ok(js_object)
}

pub(crate) fn js_optional_object_from_teo_object(env: Env, teo_model_object: Option<model::Object>) -> Result<JsUnknown> {
    Ok(match teo_model_object {
        Some(teo_model_object) => js_model_object_from_teo_model_object(env, teo_model_object)?.into_unknown(),
        None => env.get_undefined()?.into_unknown(),
    })
}

pub(crate) fn js_ctx_object_from_teo_transaction_ctx(env: Env, transaction_ctx: transaction::Ctx, name: &str) -> Result<JsObject> {
    let mut js_object = env.create_object()?;
    let prototype = get_ctx_prototype(env.clone(), name);
    js_object.set_named_property("__proto__", prototype)?;
    env.wrap(&mut js_object, transaction_ctx)?;
    Ok(js_object)
}

pub(crate) fn synthesize_dynamic_nodejs_classes(app: &App, env: Env) -> Result<()> {
    synthesize_dynamic_nodejs_classes_for_namespace(app.main_namespace(), env)
}

pub(crate) fn synthesize_dynamic_nodejs_classes_for_namespace(namespace: &'static Namespace, env: Env) -> Result<()> {
    synthesize_direct_dynamic_nodejs_classes_for_namespace(namespace, env)?;
    for namespace in namespace.namespaces.values() {
        synthesize_dynamic_nodejs_classes_for_namespace(namespace, env)?;
    }
    Ok(())
}

pub(crate) fn synthesize_direct_dynamic_nodejs_classes_for_namespace(namespace: &'static Namespace, env: Env) -> Result<()> {
    let ctx_ctor = ctx_constructor_function(env, &namespace.path().join("."))?;
    let ctx_ctor_object = ctx_ctor.coerce_to_object()?;
    let mut ctx_prototype: JsObject = ctx_ctor_object.get_named_property("prototype")?;
    for model in namespace.models.values() {
        let model_name = model.path().join(".");
        let lowercase_model_name = model_name.to_lowercase();
        let ctx_property = Property::new(&lowercase_model_name)?.with_getter_closure(|env: Env, this: JsObject| {
            let model_name = model.path().join(".");
            let transaction_ctx: &mut transaction::Ctx = env.unwrap(&this)?;
            let model_ctx = transaction_ctx.model_ctx_for_model_at_path(&model.path()).unwrap();
            js_model_class_object_from_teo_model_ctx(env, model_ctx, &model_name)
        });
        ctx_prototype.define_properties(&[ctx_property])?;
        let class_ctor = get_model_class_constructor(env, &model_name);
        let class_ctor_object = class_ctor.coerce_to_object()?;
        let mut class_prototype: JsObject = class_ctor_object.get_named_property("prototype")?;
        let object_ctor = get_model_object_constructor(env, &model_name);
        let object_ctor_object = object_ctor.coerce_to_object()?;
        let mut object_prototype: JsObject = object_ctor_object.get_named_property("prototype")?;
        // find unique
        let find_unique = env.create_function_from_closure("findUnique", |ctx| {
            let teo_value = if ctx.length == 0 {
                TeoValue::Dictionary(IndexMap::new())
            } else {
                let unknown: JsUnknown = ctx.get(0)?;
                js_any_to_teo_object(unknown, ctx.env.clone())?.as_teon().unwrap().clone()
            };
            let this: JsObject = ctx.this()?;
            let model_ctx: &mut model::Ctx = ctx.env.unwrap(&this)?;
            let model_ctx_clone = model_ctx.clone();
            let promise = ctx.env.execute_tokio_future((|| async move {
                match model_ctx_clone.find_unique(&teo_value).await {
                    Ok(obj) => Ok(obj),
                    Err(err) => Err(Error::from_reason(err.message())),
                }
            })(), |env, object: Option<model::Object>| {
                js_optional_object_from_teo_object(env.clone(), object)
            })?;
            Ok(promise)
        })?;
        class_prototype.set_named_property("findUnique", find_unique)?;
        // find first
        let find_unique = env.create_function_from_closure("findFirst", |ctx| {
            let teo_value = if ctx.length == 0 {
                TeoValue::Dictionary(IndexMap::new())
            } else {
                let unknown: JsUnknown = ctx.get(0)?;
                js_any_to_teo_object(unknown, ctx.env.clone())?.as_teon().unwrap().clone()
            };
            let this: JsObject = ctx.this()?;
            let model_ctx: &mut model::Ctx = ctx.env.unwrap(&this)?;
            let model_ctx_cloned = model_ctx.clone();
            let promise = ctx.env.execute_tokio_future((|| async move {
                match model_ctx_cloned.find_first(&teo_value).await {
                    Ok(obj) => Ok(obj),
                    Err(err) => Err(Error::from_reason(err.message())),
                }
            })(), |env, object: Option<model::Object>| {
                js_optional_object_from_teo_object(env.clone(), object)
            })?;
            Ok(promise)
        })?;
        class_prototype.set_named_property("findFirst", find_unique)?;
        // find many
        let find_many = env.create_function_from_closure("findMany", |ctx| {
            let teo_value = if ctx.length == 0 {
                TeoValue::Dictionary(IndexMap::new())
            } else {
                let unknown: JsUnknown = ctx.get(0)?;
                js_any_to_teo_object(unknown, ctx.env.clone())?.as_teon().unwrap().clone()
            };
            let this: JsObject = ctx.this()?;
            let model_ctx: &mut model::Ctx = ctx.env.unwrap(&this)?;
            let model_ctx_cloned = model_ctx.clone();
            let promise = ctx.env.execute_tokio_future((|| async move {
                match model_ctx_cloned.find_many(&teo_value).await {
                    Ok(objects) => Ok(objects),
                    Err(err) => Err(Error::from_reason(err.message())),
                }
            })(), |env, objects: Vec<model::Object>| {
                let mut array = env.create_array_with_length(objects.len())?;
                for (index, object) in objects.iter().enumerate() {
                    array.set_element(index as u32, js_model_object_from_teo_model_object(env.clone(), object.clone())?)?;
                }
                Ok(array)
            })?;
            Ok(promise)
        })?;
        class_prototype.set_named_property("findMany", find_many)?;
        // create
        let create = env.create_function_from_closure("create", |ctx| {
            let teo_value = if ctx.length == 0 {
                TeoValue::Dictionary(IndexMap::new())
            } else {
                let unknown: JsUnknown = ctx.get(0)?;
                js_any_to_teo_object(unknown, ctx.env.clone())?.as_teon().unwrap().clone()
            };
            let this: JsObject = ctx.this()?;
            let model_ctx: &mut model::Ctx = ctx.env.unwrap(&this)?;
            let model_ctx_cloned = model_ctx.clone();
            let promise = ctx.env.execute_tokio_future((|| async move {
                Ok(model_ctx_cloned.create_object(&teo_value).await.unwrap())
            })(), |env, object: model::Object| {
                js_model_object_from_teo_model_object(env.clone(), object)
            })?;
            Ok(promise)
        })?;
        class_prototype.set_named_property("create", create)?;
        // isNew
        let is_new = Property::new("isNew")?.with_getter_closure(|env: Env, this: JsObject| {
            let object: &mut model::Object = env.unwrap(&this)?;
            env.get_boolean(object.is_new())
        });
        object_prototype.define_properties(&[is_new])?;
        // isModified
        let is_modified = Property::new("isModified")?.with_getter_closure(|env: Env, this: JsObject| {
            let object: &mut model::Object = env.unwrap(&this)?;
            env.get_boolean(object.is_modified())
        });
        object_prototype.define_properties(&[is_modified])?;
        // set
        let set = env.create_function_from_closure("set", |ctx| {
            let unknown: JsUnknown = ctx.get(0)?;
            let input = js_any_to_teo_object(unknown, ctx.env.clone())?.as_teon().unwrap().clone();
            let this: JsObject = ctx.this()?;
            let object: &mut model::Object = ctx.env.unwrap(&this)?;
            let object = object.clone();
            let promise = ctx.env.execute_tokio_future((|| async move {
                match object.set_teon(&input).await {
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
            let input = js_any_to_teo_object(unknown, ctx.env.clone())?.as_teon().unwrap().clone();
            let this: JsObject = ctx.this()?;
            let object: &mut model::Object = ctx.env.unwrap(&this)?;
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
            let object: &mut model::Object = ctx.env.unwrap(&this)?;
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
            let object: &mut model::Object = ctx.env.unwrap(&this)?;
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
        // inspect
        let inspect_func = env.create_function_from_closure("inspect", |ctx| {
            let require: JsFunction = ctx.env.get_global()?.get_named_property("require")?;
            let util = require.call(None, &[ctx.env.create_string("node:util").unwrap().into_unknown()])?.coerce_to_object()?;
            let inspect: JsFunction = util.get_named_property("inspect")?;    
            let this: JsObject = ctx.this()?;
            let model_object: &mut model::Object = ctx.env.unwrap(&this)?;
            let value_map = model_object.inner.value_map.lock().unwrap();
            let mut object = ctx.env.create_object()?;
            for (k, v) in value_map.iter() {
                object.set_named_property(k.as_str(), teo_value_to_js_any(v, &ctx.env)?)?;
            }
            let inspect_options: JsObject = ctx.get(1)?;
            let object_inspect: JsString = inspect.call(Some(&this), &[object, inspect_options])?.coerce_to_string()?;
            let class_name = model_object.model().path().join(".") + " " + object_inspect.into_utf8()?.as_str()?;
            Ok(ctx.env.create_string(&class_name)?)
        })?;
        let require: JsFunction = env.get_global()?.get_named_property("require")?;
        let util = require.call(None, &[env.create_string("node:util").unwrap().into_unknown()])?.coerce_to_object()?;
        let inspect: JsObject = util.get_named_property("inspect")?;
        let custom: JsSymbol = inspect.get_named_property("custom")?;
        object_prototype.set_property(custom, inspect_func)?;
        // toString
        let to_string = env.create_function_from_closure("toString", |ctx| {
            let this: JsObject = ctx.this()?;
            let object: &mut model::Object = ctx.env.unwrap(&this)?;
            let result = format!("[object {}]", object.model().path().join("."));
            ctx.env.create_string(&result)
        })?;
        object_prototype.set_named_property("toString", to_string)?;
        // fields
        for field in model.fields() {
            let field_name = Box::leak(Box::new(field.name.clone()));
            let property = Property::new(field_name.as_str())?.with_getter_closure(|env: Env, this: JsObject| {
                let object: &mut model::Object = env.unwrap(&this)?;
                let value: TeoValue = object.get_value(field_name.as_str()).unwrap();
                let teo_object = TeoObject::from(value);
                Ok(teo_object_to_js_any(&teo_object, &env)?)
            }).with_setter_closure(|env: Env, this: JsObject, arg0: JsUnknown| {
                let teo_value = js_any_to_teo_object(arg0, env.clone())?.as_teon().unwrap().clone();
                let object: &mut model::Object = env.unwrap(&this)?;
                object.set_value(field_name.as_str(), teo_value).unwrap();
                Ok(())
            });
            object_prototype.define_properties(&[property])?;
        }
        // relations
        for relation in model.relations() {
            let name_raw = Box::leak(Box::new(relation.name.clone()));
            let name = name_raw.as_str();
            if relation.is_vec {
                // get
                let get_relation = env.create_function_from_closure(&name, move |ctx: CallContext<'_>| {
                    let teo_value = if ctx.length == 0 {
                        TeoValue::Dictionary(IndexMap::new())
                    } else {
                        let val: JsUnknown = ctx.get(0)?;
                        js_any_to_teo_object(val, ctx.env.clone())?.as_teon().unwrap().clone()
                    };
                    let this: JsObject = ctx.this()?;
                    let object: &mut model::Object = ctx.env.unwrap(&this)?;
                    let object_cloned = object.clone();
                    let promise = ctx.env.execute_tokio_future((|| async move {
                        match object_cloned.force_get_relation_objects(name, teo_value).await {
                            Ok(objects) => Ok(objects),
                            Err(err) => Err(Error::from_reason(err.message())),
                        }
                    })(), |env, objects| {
                        let mut array = env.create_array_with_length(objects.len())?;
                        for (index, object) in objects.iter().enumerate() {
                            array.set_element(index as u32, js_model_object_from_teo_model_object(env.clone(), object.clone())?)?;
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
                        let obj: &mut model::Object = ctx.env.unwrap(&element)?;
                        objects.push(obj.clone());
                    }
                    let this: JsObject = ctx.this()?;
                    let object: &mut model::Object = ctx.env.unwrap(&this)?;
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
                        let obj: &mut model::Object = ctx.env.unwrap(&element)?;
                        objects.push(obj.clone());
                    }
                    let this: JsObject = ctx.this()?;
                    let object: &mut model::Object = ctx.env.unwrap(&this)?;
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
                        let obj: &mut model::Object = ctx.env.unwrap(&element)?;
                        objects.push(obj.clone());
                    }
                    let this: JsObject = ctx.this()?;
                    let object: &mut model::Object = ctx.env.unwrap(&this)?;
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
                    let object: &mut model::Object = env.unwrap(&this)?;
                    let object_cloned = object.clone();
                    let promise = env.execute_tokio_future((|| async move {
                        match object_cloned.force_get_relation_object(name).await {
                            Ok(v) => Ok(v),
                            Err(err) => Err(Error::from_reason(err.message())),
                        }
                    })(), |env, object: Option<model::Object>| {
                        Ok(js_optional_object_from_teo_object(env.clone(), object)?.into_unknown())
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
                            let obj: &mut model::Object = ctx.env.unwrap(&object)?;
                            Some(obj.clone())
                        }
                        _ => None,
                    };
                    let this: JsObject = ctx.this()?;
                    let object: &mut model::Object = ctx.env.unwrap(&this)?;
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
        // properties
        for model_property in model.properties() {
            let field_name_raw = Box::leak(Box::new(model_property.name.clone()));
            let field_name = field_name_raw.as_str();
            if model_property.setter.is_some() {
                let name = "set".to_owned() + &field_name.to_pascal_case();
                let set_property = env.create_function_from_closure(&name, move |ctx: CallContext<'_>| {
                    let val: JsUnknown = ctx.get(0)?;
                    let teo_value = js_any_to_teo_object(val, ctx.env.clone())?.as_teon().unwrap().clone();
                    let this: JsObject = ctx.this()?;
                    let object: &mut model::Object = ctx.env.unwrap(&this)?;
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
            if model_property.getter.is_some() {
                let mut property = Property::new(field_name)?;
                property = property.with_getter_closure(move |env: Env, this: JsObject| {
                    let object: &mut model::Object = env.unwrap(&this)?;
                    let object_cloned = object.clone();
                    let promise = env.execute_tokio_future((|| async move {
                        match object_cloned.get_property_value(field_name).await {
                            Ok(v) => Ok(v),
                            Err(err) => Err(Error::from_reason(err.message())),
                        }
                    })(), |env, v: TeoValue| {
                        let obj = TeoObject::from(v);
                        Ok(teo_object_to_js_any(&obj, env))
                    })?;
                    Ok(promise)
                });
                object_prototype.define_properties(&[property])?;
            }
        }        
    }
    for namespace in namespace.namespaces.values() {
        let namespace_name = namespace.path().join(".");
        let ctx_property = Property::new(&namespace_name)?.with_getter_closure(|env: Env, this: JsObject| {
            let namespace_name = namespace.path().join(".");
            let transaction_ctx: &mut transaction::Ctx = env.unwrap(&this)?;
            let _ = ctx_constructor_function(env, &namespace.path().join("."))?;
            js_ctx_object_from_teo_transaction_ctx(env, transaction_ctx.clone(), namespace_name.as_str())
        });
        ctx_prototype.define_properties(&[ctx_property])?;
    }
    let transaction = env.create_function_from_closure("transaction", |ctx| {
        let func_arg: JsFunction = ctx.get(0)?;
        let wrapper_thread_safe: ThreadsafeFunction<transaction::Ctx, ErrorStrategy::Fatal> = func_arg.create_threadsafe_function(0, |ctx: ThreadSafeCallContext<transaction::Ctx>| {
            let js_ctx = js_ctx_object_from_teo_transaction_ctx(ctx.env, ctx.value, "")?;
            Ok(vec![js_ctx])
        })?;
        let wrapper_thread_safe_longlive = &*Box::leak(Box::new(wrapper_thread_safe));
        let this: JsObject = ctx.this()?;
        let wrapped_teo_ctx_shortlive: &mut transaction::Ctx = ctx.env.unwrap(&this)?;
        let wrapped_teo_ctx: &'static transaction::Ctx = unsafe { &*(wrapped_teo_ctx_shortlive as * const transaction::Ctx) };
        let promise = ctx.env.execute_tokio_future((|| async {
            wrapped_teo_ctx.run_transaction(|teo: transaction::Ctx| async {
                wrapper_thread_safe_longlive.call_async(teo).await.into_teo_path_result()?;
                Ok(())
            }).await.into_nodejs_result()?;
            Ok(0)
        })(), |env: &mut Env, _unknown: i32| {
            env.get_undefined()
        })?;
        Ok(promise)
    })?;
    ctx_prototype.set_named_property("transaction", transaction)?;
    Ok(())
}