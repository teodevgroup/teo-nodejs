
use indexmap::IndexMap;
use napi::{threadsafe_function::{ErrorStrategy, ThreadSafeCallContext, ThreadsafeFunction}, CallContext, Env, Error, JsFunction, JsObject, JsString, JsSymbol, JsUnknown, Property, Result, ValueType};
use teo::prelude::{model, traits::named::Named, transaction, App, Namespace, Value as TeoValue};
use std::sync::Arc;
use inflector::Inflector;
use crate::object::{js_any_to_teo_value, carry_over::CarryOverValue, value::teo_value_to_js_any};
use super::{builder::DynamicClassesBuilder, create::CreateDynamicClasses, dynamic::DynamicClasses, query::QueryDynamicClasses};

pub fn synthesize_dynamic_nodejs_classes(app: &App, env: Env) -> Result<()> {
    let mut dynamic_classes_builder = DynamicClassesBuilder::new();
    synthesize_dynamic_nodejs_classes_for_namespace(&mut dynamic_classes_builder, app, app.compiled_main_namespace(), env)?;
    app.app_data().set_dynamic_classes(Arc::new(dynamic_classes_builder.build()))?;
    Ok(())
}

pub(crate) fn synthesize_dynamic_nodejs_classes_for_namespace(dynamic_classes_builder: &mut DynamicClassesBuilder, app: &App, namespace: &Namespace, env: Env) -> Result<()> {
    synthesize_direct_dynamic_nodejs_classes_for_namespace(dynamic_classes_builder, app, namespace, env)?;
    for namespace in namespace.namespaces().values() {
        synthesize_dynamic_nodejs_classes_for_namespace(dynamic_classes_builder, app, namespace, env)?;
    }
    Ok(())
}

pub(crate) fn synthesize_direct_dynamic_nodejs_classes_for_namespace(dynamic_classes_builder: &mut DynamicClassesBuilder, app: &App, namespace: &Namespace, env: Env) -> Result<()> {
    let ctx_ctor = dynamic_classes_builder.ctx_constructor_function_or_create(&namespace.path().join("."), env)?;
    let ctx_ctor_object = ctx_ctor.coerce_to_object()?;
    let mut ctx_prototype: JsObject = ctx_ctor_object.get_named_property("prototype")?;
    let app_data = namespace.app_data().clone();
    for model in namespace.models().values() {
        let model_name = model.path().join(".");
        let lowercase_model_name = model_name.to_camel_case();
        let ctx_property = Property::new(&lowercase_model_name)?.with_getter_closure({
            let model = model.clone();
            let app_data = app_data.clone();
            move |env: Env, this: JsObject| {
                let model = model.clone();
                let model_name = model.path().join(".");
                let transaction_ctx: &mut transaction::Ctx = env.unwrap(&this)?;
                let dynamic_classes = DynamicClasses::retrieve(&app_data)?;
                let model_ctx = transaction_ctx.model_ctx_for_model_at_path(&model.path()).unwrap();
                dynamic_classes.teo_model_ctx_to_js_model_class_object(env, model_ctx, &model_name)
            }
        });
        ctx_prototype.define_properties(&[ctx_property])?;
        let class_ctor = dynamic_classes_builder.class_constructor_function_or_create(&model_name, env)?;
        let class_ctor_object = class_ctor.coerce_to_object()?;
        let mut class_prototype: JsObject = class_ctor_object.get_named_property("prototype")?;
        let object_ctor = dynamic_classes_builder.object_constructor_function_or_create(&model_name, env)?;
        let object_ctor_object = object_ctor.coerce_to_object()?;
        let mut object_prototype: JsObject = object_ctor_object.get_named_property("prototype")?;
        // find unique
        let find_unique = env.create_function_from_closure("findUniqueObject", {
            let app_data = app_data.clone();
            move |ctx| {
                let app_data = app_data.clone();
                let teo_value = if ctx.length == 0 {
                    TeoValue::Dictionary(IndexMap::new())
                } else {
                    let unknown: JsUnknown = ctx.get(0)?;
                    js_any_to_teo_value(unknown, ctx.env.clone())?
                };
                let this: JsObject = ctx.this()?;
                let model_ctx: &mut model::Ctx = ctx.env.unwrap(&this)?;
                let model_ctx_clone = model_ctx.clone();
                let promise = ctx.env.execute_tokio_future((|| async move {
                    match model_ctx_clone.find_unique(&teo_value).await {
                        Ok(obj) => Ok(obj),
                        Err(err) => Err(Error::from_reason(err.message())),
                    }
                })(), move |env, object: Option<model::Object>| {
                    let dynamic_classes = DynamicClasses::retrieve(&app_data)?;
                    dynamic_classes.teo_optional_model_object_to_js_optional_model_object_object(env.clone(), object)
                })?;
                Ok(promise)
            }
        })?;
        class_prototype.set_named_property("findUniqueObject", find_unique)?;
        // find first
        let find_unique = env.create_function_from_closure("findFirstObject", {
            let app_data = app_data.clone();
            move |ctx| {
                let app_data = app_data.clone();
                let teo_value = if ctx.length == 0 {
                    TeoValue::Dictionary(IndexMap::new())
                } else {
                    let unknown: JsUnknown = ctx.get(0)?;
                    js_any_to_teo_value(unknown, ctx.env.clone())?
                };
                let this: JsObject = ctx.this()?;
                let model_ctx: &mut model::Ctx = ctx.env.unwrap(&this)?;
                let model_ctx_cloned = model_ctx.clone();
                let promise = ctx.env.execute_tokio_future((|| async move {
                    match model_ctx_cloned.find_first(&teo_value).await {
                        Ok(obj) => Ok(obj),
                        Err(err) => Err(Error::from_reason(err.message())),
                    }
                })(), move |env, object: Option<model::Object>| {
                    let dynamic_classes = DynamicClasses::retrieve(&app_data)?;
                    dynamic_classes.teo_optional_model_object_to_js_optional_model_object_object(env.clone(), object)
                })?;
                Ok(promise)
            }
        })?;
        class_prototype.set_named_property("findFirstObject", find_unique)?;
        // find many
        let find_many = env.create_function_from_closure("findManyObjects", {
            let app_data = app_data.clone();
            move |ctx| {
                let app_data = app_data.clone();
                let teo_value = if ctx.length == 0 {
                    TeoValue::Dictionary(IndexMap::new())
                } else {
                    let unknown: JsUnknown = ctx.get(0)?;
                    js_any_to_teo_value(unknown, ctx.env.clone())?
                };
                let this: JsObject = ctx.this()?;
                let model_ctx: &mut model::Ctx = ctx.env.unwrap(&this)?;
                let model_ctx_cloned = model_ctx.clone();
                let promise = ctx.env.execute_tokio_future((|| async move {
                    match model_ctx_cloned.find_many(&teo_value).await {
                        Ok(objects) => Ok(objects),
                        Err(err) => Err(Error::from_reason(err.message())),
                    }
                })(), move |env, objects: Vec<model::Object>| {
                    let mut array = env.create_array_with_length(objects.len())?;
                    let dynamic_classes = DynamicClasses::retrieve(&app_data)?;
                    for (index, object) in objects.iter().enumerate() {
                        array.set_element(index as u32, dynamic_classes.teo_model_object_to_js_model_object_object(env.clone(), object.clone())?)?;
                    }
                    Ok(array)
                })?;
                Ok(promise)
            }
        })?;
        class_prototype.set_named_property("findManyObjects", find_many)?;
        // create
        let create = env.create_function_from_closure("createObject", {
            let app_data = app_data.clone();
            move |ctx| {
                let app_data = app_data.clone();
                let teo_value = if ctx.length == 0 {
                    TeoValue::Dictionary(IndexMap::new())
                } else {
                    let unknown: JsUnknown = ctx.get(0)?;
                    js_any_to_teo_value(unknown, ctx.env.clone())?
                };
                let this: JsObject = ctx.this()?;
                let model_ctx: &mut model::Ctx = ctx.env.unwrap(&this)?;
                let model_ctx_cloned = model_ctx.clone();
                let promise = ctx.env.execute_tokio_future((|| async move {
                    Ok(model_ctx_cloned.create_object(&teo_value).await.unwrap())
                })(), move |env, object: model::Object| {
                    let dynamic_classes = DynamicClasses::retrieve(&app_data)?;
                    dynamic_classes.teo_model_object_to_js_model_object_object(env.clone(), object)
                })?;
                Ok(promise)
            }
        })?;
        class_prototype.set_named_property("createObject", create)?;
        // count
        let count = env.create_function_from_closure("count", {
            let app_data = app_data.clone();
            move |ctx| {
                let app_data = app_data.clone();
                let teo_value = if ctx.length == 0 {
                    TeoValue::Dictionary(IndexMap::new())
                } else {
                    let unknown: JsUnknown = ctx.get(0)?;
                    js_any_to_teo_value(unknown, ctx.env.clone())?
                };
                let this: JsObject = ctx.this()?;
                let model_ctx: &mut model::Ctx = ctx.env.unwrap(&this)?;
                let model_ctx_cloned = model_ctx.clone();
                let promise = ctx.env.execute_tokio_future((|| async move {
                    Ok(model_ctx_cloned.count(&teo_value).await.unwrap())
                })(), move |env, object: TeoValue| {
                    let app_data = app_data.clone();
                    let dynamic_classes = DynamicClasses::retrieve(&app_data)?;
                    teo_value_to_js_any(&dynamic_classes,&object, env)
                })?;
                Ok(promise)
            }
        })?;
        class_prototype.set_named_property("count", count)?;
        // aggregate
        let aggregate = env.create_function_from_closure("aggregate", {
            let app_data = app_data.clone();
            move |ctx| {
                let app_data = app_data.clone();
                let teo_value = if ctx.length == 0 {
                    TeoValue::Dictionary(IndexMap::new())
                } else {
                    let unknown: JsUnknown = ctx.get(0)?;
                    js_any_to_teo_value(unknown, ctx.env.clone())?
                };
                let this: JsObject = ctx.this()?;
                let model_ctx: &mut model::Ctx = ctx.env.unwrap(&this)?;
                let model_ctx_cloned = model_ctx.clone();
                let promise = ctx.env.execute_tokio_future((|| async move {
                    Ok(model_ctx_cloned.aggregate(&teo_value).await.unwrap())
                })(), move |env, object: TeoValue| {
                    let app_data = app_data.clone();
                    let dynamic_classes = DynamicClasses::retrieve(&app_data)?;
                    teo_value_to_js_any(&dynamic_classes, &object, env)
                })?;
                Ok(promise)
            }
        })?;
        class_prototype.set_named_property("aggregate", aggregate)?;
        // groupBy
        let group_by = env.create_function_from_closure("groupBy", {
            let app_data = app_data.clone();
            move |ctx| {
                let app_data = app_data.clone();
                let teo_value = if ctx.length == 0 {
                    TeoValue::Dictionary(IndexMap::new())
                } else {
                    let unknown: JsUnknown = ctx.get(0)?;
                    js_any_to_teo_value(unknown, ctx.env.clone())?
                };
                let this: JsObject = ctx.this()?;
                let model_ctx: &mut model::Ctx = ctx.env.unwrap(&this)?;
                let model_ctx_cloned = model_ctx.clone();
                let promise = ctx.env.execute_tokio_future((|| async move {
                    Ok(model_ctx_cloned.group_by(&teo_value).await.unwrap())
                })(), move |env, values: Vec<TeoValue>| {
                    let app_data = app_data.clone();
                    let dynamic_classes = DynamicClasses::retrieve(&app_data)?;
                    let mut array = env.create_array(values.len() as u32)?;
                    for value in values {
                        array.insert(teo_value_to_js_any(&dynamic_classes, &value, env)?)?;
                    }
                    Ok(array)
                })?;
                Ok(promise)
            }
        })?;
        class_prototype.set_named_property("groupBy", group_by)?;
        if namespace.database().is_some() && namespace.database().unwrap().is_sql() {
            // sql
            let sql = env.create_function_from_closure("sql", {
                let app_data = app_data.clone();
                move |ctx| {
                    let app_data = app_data.clone();
                    let sql_string: &str = ctx.get(0)?;
                    let this: JsObject = ctx.this()?;
                    let model_ctx: &mut model::Ctx = ctx.env.unwrap(&this)?;
                    let model_ctx_cloned = model_ctx.clone();
                    let promise = ctx.env.execute_tokio_future((|| async move {
                        Ok(model_ctx_cloned.sql(sql_string).await.unwrap())
                    })(), move |env, values: Vec<TeoValue>| {
                        let app_data = app_data.clone();
                        let dynamic_classes = DynamicClasses::retrieve(&app_data)?;
                        let mut array = env.create_array(values.len() as u32)?;
                        for value in values {
                            array.insert(teo_value_to_js_any(&dynamic_classes, &value, env)?)?;
                        }
                        Ok(array)
                    })?;
                    Ok(promise)
                }
            })?;
            class_prototype.set_named_property("sql", sql)?;
        }
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
            let input = js_any_to_teo_value(unknown, ctx.env.clone())?;
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
            let input = js_any_to_teo_value(unknown, ctx.env.clone())?;
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
        // toTeon
        let to_teon = env.create_function_from_closure("toTeon", {
            let app_data = app_data.clone();
            move |ctx| {
                let app_data = app_data.clone();
                let this: JsObject = ctx.this()?;
                let object: &mut model::Object = ctx.env.unwrap(&this)?;
                let object = object.clone();
                let promise = ctx.env.execute_tokio_future((|| async move {
                    match object.to_teon().await {
                        Ok(value) => Ok(value),
                        Err(err) => Err(Error::from_reason(err.message())),
                    }
                })(), move |env: &mut Env, value: TeoValue| {
                    let app_data = app_data.clone();
                    let dynamic_classes = DynamicClasses::retrieve(&app_data)?;
                    Ok(teo_value_to_js_any(&dynamic_classes, &value, env)?)
                })?;
                Ok(promise)
            }
        })?;
        object_prototype.set_named_property("toTeon", to_teon)?;
        // inspect
        let inspect_func = env.create_function_from_closure("inspect", {
            let app_data = app_data.clone();
            move |ctx| {
                let app_data = app_data.clone();
                let dynamic_classes = DynamicClasses::retrieve(&app_data)?;
                let require: JsFunction = ctx.env.get_global()?.get_named_property("require")?;
                let util = require.call(None, &[ctx.env.create_string("node:util").unwrap().into_unknown()])?.coerce_to_object()?;
                let inspect: JsFunction = util.get_named_property("inspect")?;    
                let this: JsObject = ctx.this()?;
                let model_object: &mut model::Object = ctx.env.unwrap(&this)?;
                let value_map = model_object.inner.value_map.lock().unwrap();
                let mut object = ctx.env.create_object()?;
                for (k, v) in value_map.iter() {
                    object.set_named_property(k.as_str(), teo_value_to_js_any(&dynamic_classes, v, &ctx.env)?)?;
                }
                let inspect_options: JsObject = ctx.get(1)?;
                let object_inspect: JsString = inspect.call(Some(&this), &[object, inspect_options])?.coerce_to_string()?;
                let class_name = model_object.model().path().join(".") + " " + object_inspect.into_utf8()?.as_str()?;
                Ok(ctx.env.create_string(&class_name)?)
            }
        })?;
        let require: JsFunction = env.get_global()?.get_named_property("require")?;
        let util = require.call(None, &[env.create_string("node:util").unwrap().into_unknown()])?.coerce_to_object()?;
        let inspect_f: JsFunction = util.get_named_property("inspect")?;
        let inspect: JsObject = inspect_f.coerce_to_object()?;
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
        for field in model.fields().values() {
            let field_name = field.name().to_string();
            let property = Property::new(field_name.as_str())?.with_getter_closure({
                let app_data = app_data.clone();
                let field_name = field_name.clone();
                move |env: Env, this: JsObject| {
                    let field_name = field_name.clone();
                    let app_data = app_data.clone();
                    let dynamic_classes = DynamicClasses::retrieve(&app_data)?;
                    let object: &mut model::Object = env.unwrap(&this)?;
                    let value: TeoValue = object.get_value(field_name.as_str()).unwrap();
                    Ok(teo_value_to_js_any(&dynamic_classes, &value, &env)?)
                }
            }).with_setter_closure({
                let field_name = field_name.clone();
                move |env: Env, this: JsObject, arg0: JsUnknown| {
                    let teo_value = js_any_to_teo_value(arg0, env.clone())?;
                    let object: &mut model::Object = env.unwrap(&this)?;
                    object.set_value(field_name.as_str(), teo_value).unwrap();
                    Ok(())
                }
            });
            object_prototype.define_properties(&[property])?;
        }
        // relations
        for relation in model.relations().values() {
            let relation_name = relation.name().to_string();
            if relation.is_vec() {
                // get
                let get_relation = env.create_function_from_closure(&relation_name, {
                    let relation_name = relation_name.clone();
                    let app_data = app_data.clone();
                    move |ctx: CallContext<'_>| {
                        let app_data = app_data.clone();
                        let relation_name = relation_name.clone();
                        let teo_value = if ctx.length == 0 {
                            TeoValue::Dictionary(IndexMap::new())
                        } else {
                            let val: JsUnknown = ctx.get(0)?;
                            js_any_to_teo_value(val, ctx.env.clone())?
                        };
                        let this: JsObject = ctx.this()?;
                        let object: &mut model::Object = ctx.env.unwrap(&this)?;
                        let object = object.clone();
                        let promise = ctx.env.execute_tokio_future((|| async move {
                            match object.force_get_relation_objects(&relation_name, teo_value).await {
                                Ok(objects) => Ok(objects),
                                Err(err) => Err(Error::from_reason(err.message())),
                            }
                        })(), move |env, objects| {
                            let app_data = app_data.clone();
                            let dynamic_classes = DynamicClasses::retrieve(&app_data)?;
                            let mut array = env.create_array_with_length(objects.len())?;
                            for (index, object) in objects.iter().enumerate() {
                                array.set_element(index as u32, &dynamic_classes.teo_model_object_to_js_model_object_object(env.clone(), object.clone())?)?;
                            }
                            Ok(array)
                        })?;
                        Ok(promise)
                    }
                })?;
                object_prototype.set_named_property(&relation_name, get_relation)?;
                // set
                let set_name = "set".to_owned() + &relation_name.to_pascal_case();
                let set_relation = env.create_function_from_closure(&relation_name, {
                    let relation_name = relation_name.clone();
                    move |ctx: CallContext<'_>| {
                        let relation_name = relation_name.clone();
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
                        let promise = ctx.env.execute_tokio_future((move || {
                            let relation_name = relation_name.clone();
                            async move {
                                Ok(object_cloned.force_set_relation_objects(&relation_name, objects).await)
                            }
                        })(), |env, _objects| {
                            env.get_undefined()
                        })?;
                        Ok(promise)
                    }
                })?;
                object_prototype.set_named_property(&set_name, set_relation)?;
                // add
                let add_name = "addTo".to_owned() + &relation_name.to_pascal_case();
                let add_relation = env.create_function_from_closure(&relation_name, {
                    let relation_name = relation_name.clone();
                    move |ctx: CallContext<'_>| {
                        let relation_name = relation_name.clone();
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
                        let promise = ctx.env.execute_tokio_future((move || {
                            let relation_name = relation_name.clone();
                            async move {
                                Ok(object_cloned.force_add_relation_objects(&relation_name, objects).await)
                            }
                        })(), |env, _objects| {
                            env.get_undefined()
                        })?;
                        Ok(promise)
                    }
                })?;
                object_prototype.set_named_property(&add_name, add_relation)?;
                // remove
                let remove_name = "removeFrom".to_owned() + &relation_name.to_pascal_case();
                let remove_relation = env.create_function_from_closure(&relation_name, {
                    let relation_name = relation_name.clone();
                    move |ctx: CallContext<'_>| {
                        let relation_name = relation_name.clone();
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
                            Ok(object_cloned.force_add_relation_objects(&relation_name, objects).await)
                        })(), |env, _objects| {
                            env.get_undefined()
                        })?;
                        Ok(promise)
                    }
                })?;
                object_prototype.set_named_property(&remove_name, remove_relation)?;
            } else {
                // get
                let mut property = Property::new(&relation_name)?;
                property = property.with_getter_closure({
                    let relation_name = relation_name.clone();
                    let app_data = app_data.clone();
                    move |env: Env, this: JsObject| {
                        let app_data = app_data.clone();
                        let relation_name = relation_name.clone();
                        let object: &mut model::Object = env.unwrap(&this)?;
                        let object = object.clone();
                        let promise = env.execute_tokio_future((move || {
                            let relation_name = relation_name.clone();
                            async move {
                                match object.force_get_relation_object(&relation_name).await {
                                    Ok(v) => Ok(v),
                                    Err(err) => Err(Error::from_reason(err.message())),
                                }    
                            }
                        })(), move |env, object: Option<model::Object>| {
                            let app_data = app_data.clone();
                            let dynamic_classes = DynamicClasses::retrieve(&app_data)?;
                            Ok(dynamic_classes.teo_optional_model_object_to_js_optional_model_object_object(env.clone(), object)?.into_unknown())
                        })?;
                        Ok(promise)
                    }
                });
                object_prototype.define_properties(&[property])?;
                // set
                let set_name = "set".to_owned() + &relation_name.to_pascal_case();
                let set_relation = env.create_function_from_closure(&relation_name, {
                    let relation_name = relation_name.clone();
                    move |ctx: CallContext<'_>| {
                        let relation_name = relation_name.clone();
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
                            Ok(object_cloned.force_set_relation_object(&relation_name, arg).await)
                        })(), |env, _objects| {
                            env.get_undefined()
                        })?;
                        Ok(promise)
                    }
                })?;
                object_prototype.set_named_property(&set_name, set_relation)?;
            }
        }
        // properties
        for model_property in model.properties().values() {
            let property_name = model_property.name().to_string();
            if model_property.setter().is_some() {
                let name = "set".to_owned() + &property_name.to_pascal_case();
                let set_property = env.create_function_from_closure(&name, {
                    let property_name = property_name.clone();
                    move |ctx: CallContext<'_>| {
                        let property_name = property_name.clone();
                        let val: JsUnknown = ctx.get(0)?;
                        let teo_value = js_any_to_teo_value(val, ctx.env.clone())?;
                        let this: JsObject = ctx.this()?;
                        let object: &mut model::Object = ctx.env.unwrap(&this)?;
                        let object = object.clone();
                        let promise = ctx.env.execute_tokio_future((move || {
                            let property_name = property_name.clone();
                            async move {
                                match object.set_property(&property_name, teo_value).await {
                                    Ok(()) => Ok(()),
                                    Err(err) => Err(Error::from_reason(err.message())),
                                }    
                            }
                        })(), |_env, v: ()| {
                            Ok(v)
                        })?;
                        Ok(promise)
                    }
                })?;
                object_prototype.set_named_property(&name, set_property)?;
            }
            if model_property.getter().is_some() {
                let mut property = Property::new(&property_name)?;
                property = property.with_getter_closure({
                    let app_data = app_data.clone();
                    move |env: Env, this: JsObject| {
                        let app_data = app_data.clone();
                        let property_name = property_name.clone();
                        let object: &mut model::Object = env.unwrap(&this)?;
                        let object = object.clone();
                        let promise = env.execute_tokio_future((move || {
                            async move {
                                match object.get_property_value(&property_name).await {
                                    Ok(v) => Ok(v),
                                    Err(err) => Err(Error::from_reason(err.message())),
                                }    
                            }
                        })(), move |env, v: TeoValue| {
                            let app_data = app_data.clone();
                            let dynamic_classes = DynamicClasses::retrieve(&app_data)?;
                            Ok(teo_value_to_js_any(&dynamic_classes, &v, env))
                        })?;
                        Ok(promise)
                    }
                });
                object_prototype.define_properties(&[property])?;
            }
        }        
    }
    for namespace in namespace.namespaces().values() {
        let namespace = namespace.clone();
        let namespace_name = namespace.path().join(".");
        let _ = dynamic_classes_builder.ctx_constructor_function_or_create(&namespace.path().join("."), env)?;
        let ctx_property = Property::new(&namespace_name)?.with_getter_closure({
            let app_data = app_data.clone();
            move |env: Env, this: JsObject| {
                let dynamic_classes = DynamicClasses::retrieve(&app_data)?;
                let namespace_name = namespace.path().join(".");
                let transaction_ctx: &mut transaction::Ctx = env.unwrap(&this)?;
                dynamic_classes.teo_transaction_ctx_to_js_ctx_object(env, transaction_ctx.clone(), namespace_name.as_str())
            }
        });
        ctx_prototype.define_properties(&[ctx_property])?;
    }
    let _transaction = env.create_function_from_closure("_transaction", {
        let app_data = app_data.clone();
        move |ctx| {
            let app_data = app_data.clone();
            let callback: JsFunction = ctx.get(0)?;
            let threadsafe_callback: ThreadsafeFunction<transaction::Ctx, ErrorStrategy::CalleeHandled> = callback.create_threadsafe_function(0, move |ctx: ThreadSafeCallContext<transaction::Ctx>| {
                let dynamic_classes = DynamicClasses::retrieve(&app_data)?;
                let js_ctx = dynamic_classes.teo_transaction_ctx_to_js_ctx_object(ctx.env, ctx.value, "")?;
                Ok(vec![js_ctx])
            })?;
            let this: JsObject = ctx.this()?;
            let teo_ctx: &mut transaction::Ctx = ctx.env.unwrap(&this)?;
            let teo_ctx = teo_ctx.clone();
            let promise = ctx.env.execute_tokio_future((move || {
                let threadsafe_callback = threadsafe_callback.clone();
                let teo_ctx = teo_ctx.clone();
                async move {
                    let result = teo_ctx.run_transaction(|teo: transaction::Ctx| async {
                        let retval: CarryOverValue = threadsafe_callback.call_async(Ok(teo)).await?;
                        Ok(retval)
                    }).await?;
                    Ok(result)
                }
            })(), |_: &mut Env, reference: CarryOverValue| {
                Ok(reference)
            })?;
            Ok(promise)
        }
    })?;
    ctx_prototype.set_named_property("_transaction", _transaction)?;
    let global = env.get_global()?;
    let require: JsFunction = global.get_named_property("require")?;
    let module = require.call(None, &[env.create_string("@teodevgroup/teo-nodejs-helpers")?])?.coerce_to_object()?;
    let fixer: JsFunction = module.get_named_property("fixTransactionCallback")?;
    fixer.call(None, &[&ctx_prototype])?;
    Ok(())
}