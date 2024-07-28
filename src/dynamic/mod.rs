use indexmap::IndexMap;
use napi::{threadsafe_function::{ErrorStrategy, ThreadSafeCallContext, ThreadsafeFunction}, CallContext, Env, Error, JsFunction, JsObject, JsString, JsSymbol, JsUnknown, Property, Result, ValueType};
use teo::prelude::{app::data::AppData, model, traits::named::Named, transaction, App, Namespace, Value as TeoValue};
use std::{collections::BTreeMap, sync::Arc};
use inflector::Inflector;
use crate::object::{js_any_to_teo_value, unknown::{SendJsUnknown, SendJsUnknownOrPromise}, value::teo_value_to_js_any};

pub(crate) struct JSClassLookupMap {
    ctxs: BTreeMap<String, napi::Ref<()>>,
    classes: BTreeMap<String, napi::Ref<()>>,
    objects: BTreeMap<String, napi::Ref<()>>,
}

impl JSClassLookupMap {

    pub(crate) fn from_app_data(app_data: &AppData) -> &'static Self {
        unsafe {
            let pointer = app_data.dynamic_classes_pointer() as * mut Self;
            &*pointer as &JSClassLookupMap
        }
    }

    fn new() -> Self {
        Self {
            ctxs: BTreeMap::new(),
            classes: BTreeMap::new(),
            objects: BTreeMap::new(),
        }
    }

    fn ctxs(&self) -> &BTreeMap<String, napi::Ref<()>> {
        &self.ctxs
    }

    fn classes(&self) -> &BTreeMap<String, napi::Ref<()>> {
        &self.classes
    }

    fn objects(&self) -> &BTreeMap<String, napi::Ref<()>> {
        &self.objects
    }

    // Building methods

    fn insert_ctx(&mut self, key: String, value: napi::Ref<()>) {
        self.ctxs.insert(key, value);
    }

    fn insert_class(&mut self, key: String, value: napi::Ref<()>) {
        self.classes.insert(key, value);
    }

    fn insert_object(&mut self, key: String, value: napi::Ref<()>) {
        self.objects.insert(key, value);
    }

    fn ctx_constructor(&self, key: &str, env: Env) -> Result<Option<JsFunction>> {
        match self.ctxs.get(key).map(|r| {
            let function: Result<JsFunction> = env.get_reference_value(r);
            function
        }) {
            None => Ok(None),
            Some(result) => match result {
                Ok(function) => Ok(Some(function)),
                Err(e) => Err(e),
            },
        }
    }

    fn ctx_constructor_or_create(&mut self, key: &str, env: Env) -> Result<JsFunction> {
        let existing = self.ctx_constructor(key, env)?;
        if existing.is_some() {
            return Ok(existing.unwrap())
        }
        self.create_ctx_constructor(key, env)
    }

    fn create_ctx_constructor(&mut self, name: &str, env: Env) -> Result<JsFunction> {
        let ctor = env.create_function_from_closure((name.to_owned() + "Ctx").as_str(), |ctx| {
            ctx.env.get_undefined()
        })?;
        let mut prototype = env.create_object()?;
        prototype.set_named_property("__teo_ctx__", env.get_boolean(true)?)?;
        let mut ctor_object = ctor.coerce_to_object()?;
        ctor_object.set_named_property("prototype", prototype)?;
        let r = env.create_reference(ctor_object)?;
        self.ctxs.insert(name.to_owned(), r);
        Ok(self.ctx_constructor(name, env)?.unwrap())
    }

    fn ctx_prototype_or_create(&mut self, key: &str, env: Env) -> Result<JsObject> {
        let ctx_ctor = self.ctx_constructor_or_create(key, env)?;
        let ctx_ctor_object = ctx_ctor.coerce_to_object()?;
        let prototype: JsObject = ctx_ctor_object.get_named_property("prototype")?;
        Ok(prototype)
    }

    fn ctx_prototype(&self, key: &str, env: Env) -> Result<Option<JsObject>> {
        match self.ctxs.get(key).map(|r| {
            let ctor: Result<JsFunction> = env.get_reference_value(r);
            ctor
        }) {
            None => Ok(None),
            Some(result) => match result {
                Ok(ctor) => {
                    let ctor_object = ctor.coerce_to_object()?;
                    let prototype: JsObject = ctor_object.get_named_property("prototype")?;
                    Ok(Some(prototype))
                }
                Err(e) => Err(e),
            },
        }
    }

    fn class_constructor(&self, key: &str, env: Env) -> Result<Option<JsFunction>> {
        match self.classes.get(key).map(|r| {
            let function: Result<JsFunction> = env.get_reference_value(r);
            function
        }) {
            None => Ok(None),
            Some(result) => match result {
                Ok(function) => Ok(Some(function)),
                Err(e) => Err(e),
            },
        }
    }

    fn class_constructor_or_create(&mut self, key: &str, env: Env) -> Result<JsFunction> {
        let existing = self.class_constructor(key, env)?;
        if existing.is_some() {
            return Ok(existing.unwrap())
        }
        self.create_class_constructor(key, env)
    }

    fn create_class_constructor(&mut self, name: &str, env: Env) -> Result<JsFunction> {
        let ctor = env.create_function_from_closure(name, |ctx| {
            ctx.env.get_undefined()
        })?;
        let mut prototype = env.create_object()?;
        prototype.set_named_property("__teo_class__", env.get_boolean(true)?)?;
        let mut ctor_object = ctor.coerce_to_object()?;
        ctor_object.set_named_property("prototype", prototype)?;
        let r = env.create_reference(ctor_object)?;
        self.classes.insert(name.to_owned(), r);
        Ok(self.class_constructor(name, env)?.unwrap())
    }

    fn class_prototype_or_create(&mut self, key: &str, env: Env) -> Result<JsObject> {
        let class_ctor = self.class_constructor_or_create(key, env)?;
        let class_ctor_object = class_ctor.coerce_to_object()?;
        let prototype: JsObject = class_ctor_object.get_named_property("prototype")?;
        Ok(prototype)
    }

    fn class_prototype(&self, key: &str, env: Env) -> Result<Option<JsObject>> {
        match self.classes.get(key).map(|r| {
            let ctor: Result<JsFunction> = env.get_reference_value(r);
            ctor
        }) {
            None => Ok(None),
            Some(result) => match result {
                Ok(ctor) => {
                    let ctor_object = ctor.coerce_to_object()?;
                    let prototype: JsObject = ctor_object.get_named_property("prototype")?;
                    Ok(Some(prototype))
                }
                Err(e) => Err(e),
            },
        }
    }

    fn object_constructor(&self, key: &str, env: Env) -> Result<Option<JsFunction>> {
        match self.objects.get(key).map(|r| {
            let function: Result<JsFunction> = env.get_reference_value(r);
            function
        }) {
            None => Ok(None),
            Some(result) => match result {
                Ok(function) => Ok(Some(function)),
                Err(e) => Err(e),
            },
        }
    }

    fn object_constructor_or_create(&mut self, key: &str, env: Env) -> Result<JsFunction> {
        let existing = self.object_constructor(key, env)?;
        if existing.is_some() {
            return Ok(existing.unwrap())
        }
        self.create_object_constructor(key, env)
    }

    fn create_object_constructor(&mut self, name: &str, env: Env) -> Result<JsFunction> {
        let ctor = env.create_function_from_closure(&name, |ctx| {
            ctx.env.get_undefined()
        })?;
        let mut prototype = env.create_object()?;
        prototype.set_named_property("__teo_object__", env.get_boolean(true)?)?;
        let mut ctor_object = ctor.coerce_to_object()?;
        ctor_object.set_named_property("prototype", prototype)?;
        let r = env.create_reference(ctor_object)?;
        self.objects.insert(name.to_owned(), r);
        Ok(self.object_constructor(name, env)?.unwrap())
    }

    fn object_prototype_or_create(&mut self, key: &str, env: Env) -> Result<JsObject> {
        let object_ctor = self.object_constructor_or_create(key, env)?;
        let object_ctor_object = object_ctor.coerce_to_object()?;
        let prototype: JsObject = object_ctor_object.get_named_property("prototype")?;
        Ok(prototype)
    }

    fn object_prototype(&self, key: &str, env: Env) -> Result<Option<JsObject>> {
        match self.objects.get(key).map(|r| {
            let ctor: Result<JsFunction> = env.get_reference_value(r);
            ctor
        }) {
            None => Ok(None),
            Some(result) => match result {
                Ok(ctor) => {
                    let ctor_object = ctor.coerce_to_object()?;
                    let prototype: JsObject = ctor_object.get_named_property("prototype")?;
                    Ok(Some(prototype))
                }
                Err(e) => Err(e),
            },
        }
    }

    // Query methods

    pub(crate) fn teo_model_ctx_to_js_model_class_object(&self, env: Env, model_ctx: model::Ctx, name: &str) -> Result<JsObject> {
        let Some(class_prototype) = self.class_prototype(name, env)? else {
            return Err(Error::from_reason("Class prototype not found"));
        };
        let mut js_object = env.create_object()?;
        js_object.set_named_property("__proto__", class_prototype)?;
        env.wrap(&mut js_object, model_ctx)?;
        Ok(js_object)
    }

    pub(crate) fn teo_model_object_to_js_model_object_object(&self, env: Env, teo_model_object: model::Object) -> Result<JsObject> {
        let Some(object_prototype) = self.object_prototype(&teo_model_object.model().path().join("."), env)? else {
            return Err(Error::from_reason("Object prototype not found"));
        };
        let mut js_object = env.create_object()?;
        js_object.set_named_property("__proto__", object_prototype)?;
        env.wrap(&mut js_object, teo_model_object)?;
        Ok(js_object)
    }

    pub(crate) fn teo_optional_model_object_to_js_optional_model_object_object(&self, env: Env, teo_model_object: Option<model::Object>) -> Result<JsUnknown> {
        Ok(match teo_model_object {
            Some(teo_model_object) => self.teo_model_object_to_js_model_object_object(env, teo_model_object)?.into_unknown(),
            None => env.get_undefined()?.into_unknown(),
        })
    }

    pub(crate) fn teo_transaction_ctx_to_js_ctx_object(&self, env: Env, transaction_ctx: transaction::Ctx, name: &str) -> Result<JsObject> {
        let Some(ctx_prototype) = self.ctx_prototype(name, env)? else {
            return Err(Error::from_reason("Ctx prototype not found"));
        };
        let mut js_object = env.create_object()?;
        js_object.set_named_property("__proto__", ctx_prototype)?;
        env.wrap(&mut js_object, transaction_ctx)?;
        Ok(js_object)
    }
}

pub(crate) fn synthesize_dynamic_nodejs_classes(app: &App, env: Env) -> Result<()> {
    let static_app = unsafe { &*(app as *const App) } as &'static App;
    let mut map = JSClassLookupMap::new();
    synthesize_dynamic_nodejs_classes_for_namespace(&mut map, static_app, static_app.compiled_main_namespace(), env)?;
    let raw_map_pointer = Box::into_raw(Box::new(map));
    app.app_data().set_dynamic_classes_pointer(raw_map_pointer as * mut ());
    app.app_data().set_dynamic_classes_clean_up(Arc::new(|app_data: AppData| {
        unsafe {
            let raw_pointer = app_data.dynamic_classes_pointer() as * mut JSClassLookupMap;
            let _ = Box::from_raw(raw_pointer);
        }
    }));
    Ok(())
}

pub(crate) fn synthesize_dynamic_nodejs_classes_for_namespace(map: &mut JSClassLookupMap, app: &'static App, namespace: &'static Namespace, env: Env) -> Result<()> {
    synthesize_direct_dynamic_nodejs_classes_for_namespace(map, app, namespace, env)?;
    for namespace in namespace.namespaces().values() {
        synthesize_dynamic_nodejs_classes_for_namespace(map, app, namespace, env)?;
    }
    Ok(())
}

pub(crate) fn synthesize_direct_dynamic_nodejs_classes_for_namespace(map: &mut JSClassLookupMap, app: &'static App, namespace: &'static Namespace, env: Env) -> Result<()> {
    let ctx_ctor = map.ctx_constructor_or_create(&namespace.path().join("."), env)?;
    let ctx_ctor_object = ctx_ctor.coerce_to_object()?;
    let mut ctx_prototype: JsObject = ctx_ctor_object.get_named_property("prototype")?;
    let app_data: &'static AppData = app.app_data();
    for model in namespace.models().values() {
        let model_name = model.path().join(".");
        let lowercase_model_name = model_name.to_camel_case();
        let ctx_property = Property::new(&lowercase_model_name)?.with_getter_closure(move |env: Env, this: JsObject| {
            let model_name = model.path().join(".");
            let transaction_ctx: &mut transaction::Ctx = env.unwrap(&this)?;
            let model_ctx = transaction_ctx.model_ctx_for_model_at_path(&model.path()).unwrap();
            let app_map = JSClassLookupMap::from_app_data(app_data);
            app_map.teo_model_ctx_to_js_model_class_object(env, model_ctx, &model_name)
        });
        ctx_prototype.define_properties(&[ctx_property])?;
        let class_ctor = map.class_constructor_or_create(&model_name, env)?;
        let class_ctor_object = class_ctor.coerce_to_object()?;
        let mut class_prototype: JsObject = class_ctor_object.get_named_property("prototype")?;
        let object_ctor = map.object_constructor_or_create(&model_name, env)?;
        let object_ctor_object = object_ctor.coerce_to_object()?;
        let mut object_prototype: JsObject = object_ctor_object.get_named_property("prototype")?;
        // find unique
        let find_unique = env.create_function_from_closure("findUnique", move |ctx| {
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
                let app_map = JSClassLookupMap::from_app_data(app_data);
                app_map.teo_optional_model_object_to_js_optional_model_object_object(env.clone(), object)
            })?;
            Ok(promise)
        })?;
        class_prototype.set_named_property("findUnique", find_unique)?;
        // find first
        let find_unique = env.create_function_from_closure("findFirst", move |ctx| {
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
                let app_map = JSClassLookupMap::from_app_data(app_data);
                app_map.teo_optional_model_object_to_js_optional_model_object_object(env.clone(), object)
            })?;
            Ok(promise)
        })?;
        class_prototype.set_named_property("findFirst", find_unique)?;
        // find many
        let find_many = env.create_function_from_closure("findMany", move |ctx| {
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
                let app_map = JSClassLookupMap::from_app_data(app_data);
                let mut array = env.create_array_with_length(objects.len())?;
                for (index, object) in objects.iter().enumerate() {
                    array.set_element(index as u32, app_map.teo_model_object_to_js_model_object_object(env.clone(), object.clone())?)?;
                }
                Ok(array)
            })?;
            Ok(promise)
        })?;
        class_prototype.set_named_property("findMany", find_many)?;
        // create
        let create = env.create_function_from_closure("create", move |ctx| {
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
                let app_map = JSClassLookupMap::from_app_data(app_data);
                app_map.teo_model_object_to_js_model_object_object(env.clone(), object)
            })?;
            Ok(promise)
        })?;
        class_prototype.set_named_property("create", create)?;
        // count
        let count = env.create_function_from_closure("count", move |ctx| {
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
                teo_value_to_js_any(
                    app_data,
                    &object,
                    env
                )
            })?;
            Ok(promise)
        })?;
        class_prototype.set_named_property("count", count)?;
        // aggregate
        let aggregate = env.create_function_from_closure("aggregate", move |ctx| {
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
            })(), |env, object: TeoValue| {
                teo_value_to_js_any(
                    app_data,
                    &object, 
                    env
                )
            })?;
            Ok(promise)
        })?;
        class_prototype.set_named_property("aggregate", aggregate)?;
        // groupBy
        let group_by = env.create_function_from_closure("groupBy", move |ctx| {
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
            })(), |env, values: Vec<TeoValue>| {
                let mut array = env.create_array(values.len() as u32)?;
                for value in values {
                    array.insert(teo_value_to_js_any(app_data, &value, env)?)?;
                }
                Ok(array)
            })?;
            Ok(promise)
        })?;
        class_prototype.set_named_property("groupBy", group_by)?;
        if namespace.database().is_some() && namespace.database().unwrap().is_sql() {
            // sql
            let sql = env.create_function_from_closure("sql", move |ctx| {
                let sql_string: &str = ctx.get(0)?;
                let this: JsObject = ctx.this()?;
                let model_ctx: &mut model::Ctx = ctx.env.unwrap(&this)?;
                let model_ctx_cloned = model_ctx.clone();
                let promise = ctx.env.execute_tokio_future((|| async move {
                    Ok(model_ctx_cloned.sql(sql_string).await.unwrap())
                })(), |env, values: Vec<TeoValue>| {
                    let mut array = env.create_array(values.len() as u32)?;
                    for value in values {
                        array.insert(teo_value_to_js_any(app_data, &value, env)?)?;
                    }
                    Ok(array)
                })?;
                Ok(promise)
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
        let to_teon = env.create_function_from_closure("toTeon", move |ctx| {
            let this: JsObject = ctx.this()?;
            let object: &mut model::Object = ctx.env.unwrap(&this)?;
            let object = object.clone();
            let promise = ctx.env.execute_tokio_future((|| async move {
                match object.to_teon().await {
                    Ok(value) => Ok(value),
                    Err(err) => Err(Error::from_reason(err.message())),
                }
            })(), move |env: &mut Env, value: TeoValue| {
                Ok(teo_value_to_js_any(app_data, &value, env)?)
            })?;
            Ok(promise)
        })?;
        object_prototype.set_named_property("toTeon", to_teon)?;
        // inspect
        let inspect_func = env.create_function_from_closure("inspect", |ctx| {
            let require: JsFunction = ctx.env.get_global()?.get_named_property("require")?;
            let util = require.call(None, &[ctx.env.create_string("node:util").unwrap().into_unknown()])?.coerce_to_object()?;
            let inspect: JsFunction = util.get_named_property("inspect")?;    
            let this: JsObject = ctx.this()?;
            let model_object: &mut model::Object = ctx.env.unwrap(&this)?;
            let value_map = model_object.inner.value_map.lock().unwrap();
            let mut object = ctx.env.create_object()?;
            let app_data = model_object.namespace().app_data();
            for (k, v) in value_map.iter() {
                object.set_named_property(k.as_str(), teo_value_to_js_any(app_data, v, &ctx.env)?)?;
            }
            let inspect_options: JsObject = ctx.get(1)?;
            let object_inspect: JsString = inspect.call(Some(&this), &[object, inspect_options])?.coerce_to_string()?;
            let class_name = model_object.model().path().join(".") + " " + object_inspect.into_utf8()?.as_str()?;
            Ok(ctx.env.create_string(&class_name)?)
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
            let field_name = Box::leak(Box::new(field.name().to_string()));
            let property = Property::new(field_name.as_str())?.with_getter_closure(|env: Env, this: JsObject| {
                let object: &mut model::Object = env.unwrap(&this)?;
                let value: TeoValue = object.get_value(field_name.as_str()).unwrap();
                Ok(teo_value_to_js_any(app_data, &value, &env)?)
            }).with_setter_closure(|env: Env, this: JsObject, arg0: JsUnknown| {
                let teo_value = js_any_to_teo_value(arg0, env.clone())?;
                let object: &mut model::Object = env.unwrap(&this)?;
                object.set_value(field_name.as_str(), teo_value).unwrap();
                Ok(())
            });
            object_prototype.define_properties(&[property])?;
        }
        // relations
        for relation in model.relations().values() {
            let name_raw = Box::leak(Box::new(relation.name().to_string()));
            let name = name_raw.as_str();
            if relation.is_vec() {
                // get
                let get_relation = env.create_function_from_closure(&name, move |ctx: CallContext<'_>| {
                    let teo_value = if ctx.length == 0 {
                        TeoValue::Dictionary(IndexMap::new())
                    } else {
                        let val: JsUnknown = ctx.get(0)?;
                        js_any_to_teo_value(val, ctx.env.clone())?
                    };
                    let this: JsObject = ctx.this()?;
                    let object: &mut model::Object = ctx.env.unwrap(&this)?;
                    let object_cloned = object.clone();
                    let promise = ctx.env.execute_tokio_future((|| async move {
                        match object_cloned.force_get_relation_objects(name, teo_value).await {
                            Ok(objects) => Ok(objects),
                            Err(err) => Err(Error::from_reason(err.message())),
                        }
                    })(), move |env, objects| {
                        let app_map = JSClassLookupMap::from_app_data(app_data);
                        let mut array = env.create_array_with_length(objects.len())?;
                        for (index, object) in objects.iter().enumerate() {
                            array.set_element(index as u32, app_map.teo_model_object_to_js_model_object_object(env.clone(), object.clone())?)?;
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
                    })(), move |env, object: Option<model::Object>| {
                        let app_map = JSClassLookupMap::from_app_data(app_data);
                        Ok(app_map.teo_optional_model_object_to_js_optional_model_object_object(env.clone(), object)?.into_unknown())
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
        for model_property in model.properties().values() {
            let field_name_raw = Box::leak(Box::new(model_property.name().to_string()));
            let field_name = field_name_raw.as_str();
            if model_property.setter().is_some() {
                let name = "set".to_owned() + &field_name.to_pascal_case();
                let set_property = env.create_function_from_closure(&name, move |ctx: CallContext<'_>| {
                    let val: JsUnknown = ctx.get(0)?;
                    let teo_value = js_any_to_teo_value(val, ctx.env.clone())?;
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
            if model_property.getter().is_some() {
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
                        Ok(teo_value_to_js_any(app_data, &v, env))
                    })?;
                    Ok(promise)
                });
                object_prototype.define_properties(&[property])?;
            }
        }        
    }
    for namespace in namespace.namespaces().values() {
        let namespace_name = namespace.path().join(".");
        let _ = map.ctx_constructor_or_create(&namespace.path().join("."), env)?;
        let ctx_property = Property::new(&namespace_name)?.with_getter_closure(move |env: Env, this: JsObject| {
            let namespace_name = namespace.path().join(".");
            let transaction_ctx: &mut transaction::Ctx = env.unwrap(&this)?;
            let app_map = JSClassLookupMap::from_app_data(app_data);
            app_map.teo_transaction_ctx_to_js_ctx_object(env, transaction_ctx.clone(), namespace_name.as_str())
        });
        ctx_prototype.define_properties(&[ctx_property])?;
    }
    let transaction = env.create_function_from_closure("transaction", move |ctx| {
        let func_arg: JsFunction = ctx.get(0)?;
        let wrapper_thread_safe: ThreadsafeFunction<transaction::Ctx, ErrorStrategy::Fatal> = func_arg.create_threadsafe_function(0, move |ctx: ThreadSafeCallContext<transaction::Ctx>| {
            let app_map = JSClassLookupMap::from_app_data(app_data);
            let js_ctx = app_map.teo_transaction_ctx_to_js_ctx_object(ctx.env, ctx.value, "")?;
            Ok(vec![js_ctx])
        })?;
        let wrapper_thread_safe_longlive = &*Box::leak(Box::new(wrapper_thread_safe));
        let this: JsObject = ctx.this()?;
        let wrapped_teo_ctx_shortlive: &mut transaction::Ctx = ctx.env.unwrap(&this)?;
        let wrapped_teo_ctx: &'static transaction::Ctx = unsafe { &*(wrapped_teo_ctx_shortlive as * const transaction::Ctx) };
        let promise = ctx.env.execute_tokio_future((|| async {
            let result = wrapped_teo_ctx.run_transaction(|teo: transaction::Ctx| async {
                let retval: SendJsUnknownOrPromise = wrapper_thread_safe_longlive.call_async(teo).await?;
                Ok(retval.to_send_js_unknown().await)
            }).await?;
            result
        })(), |_: &mut Env, unknown: SendJsUnknown| {
            Ok(unknown.inner)
        })?;
        Ok(promise)
    })?;
    ctx_prototype.set_named_property("transaction", transaction)?;
    Ok(())
}