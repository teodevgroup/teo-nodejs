use std::collections::BTreeMap;
use napi::{Env, JsFunction, JsObject, Result};
use super::fetch::FetchDynamicClasses;

pub trait CreateDynamicClasses: FetchDynamicClasses {

    fn ctxs_mut(&mut self) -> &mut BTreeMap<String, napi::Ref<()>>;

    fn classes_mut(&mut self) -> &mut BTreeMap<String, napi::Ref<()>>;

    fn objects_mut(&mut self) -> &mut BTreeMap<String, napi::Ref<()>>;

    fn ctx_constructor_function_or_create(&mut self, key: &str, env: Env) -> Result<JsFunction> {
        let constructor = self.ctx_constructor_function(key, env)?;
        if constructor.is_some() {
            return Ok(constructor.unwrap())
        }
        self.create_ctx_constructor_function(key, env)
    }

    fn create_ctx_constructor_function(&mut self, name: &str, env: Env) -> Result<JsFunction> {
        let constructor = env.create_function_from_closure((name.to_owned() + "Ctx").as_str(), |ctx| {
            ctx.env.get_undefined()
        })?;
        let mut prototype = env.create_object()?;
        prototype.set_named_property("__teo_ctx__", env.get_boolean(true)?)?;
        let mut constructor = constructor.coerce_to_object()?;
        constructor.set_named_property("prototype", prototype)?;
        let reference = env.create_reference(constructor)?;
        self.ctxs_mut().insert(name.to_owned(), reference);
        Ok(self.ctx_constructor_function(name, env)?.unwrap())
    }

    fn ctx_prototype_or_create(&mut self, key: &str, env: Env) -> Result<JsObject> {
        let constructor = self.ctx_constructor_function_or_create(key, env)?;
        let constructor = constructor.coerce_to_object()?;
        let prototype: JsObject = constructor.get_named_property("prototype")?;
        Ok(prototype)
    }

    fn class_constructor_function_or_create(&mut self, key: &str, env: Env) -> Result<JsFunction> {
        let constructor = self.class_constructor_function(key, env)?;
        if constructor.is_some() {
            return Ok(constructor.unwrap())
        }
        self.create_class_constructor_function(key, env)
    }

    fn create_class_constructor_function(&mut self, name: &str, env: Env) -> Result<JsFunction> {
        let constructor = env.create_function_from_closure(name, |ctx| {
            ctx.env.get_undefined()
        })?;
        let mut prototype = env.create_object()?;
        prototype.set_named_property("__teo_class__", env.get_boolean(true)?)?;
        let mut constructor = constructor.coerce_to_object()?;
        constructor.set_named_property("prototype", prototype)?;
        let reference = env.create_reference(constructor)?;
        self.classes_mut().insert(name.to_owned(), reference);
        Ok(self.class_constructor_function(name, env)?.unwrap())
    }

    fn class_prototype_or_create(&mut self, key: &str, env: Env) -> Result<JsObject> {
        let constructor = self.class_constructor_function_or_create(key, env)?;
        let constructor = constructor.coerce_to_object()?;
        let prototype: JsObject = constructor.get_named_property("prototype")?;
        Ok(prototype)
    }

    fn object_constructor_function_or_create(&mut self, key: &str, env: Env) -> Result<JsFunction> {
        let constructor = self.object_constructor_function(key, env)?;
        if constructor.is_some() {
            return Ok(constructor.unwrap())
        }
        self.create_object_constructor_function(key, env)
    }

    fn create_object_constructor_function(&mut self, name: &str, env: Env) -> Result<JsFunction> {
        let constructor = env.create_function_from_closure(&name, |ctx| {
            ctx.env.get_undefined()
        })?;
        let mut prototype = env.create_object()?;
        prototype.set_named_property("__teo_object__", env.get_boolean(true)?)?;
        let mut constructor = constructor.coerce_to_object()?;
        constructor.set_named_property("prototype", prototype)?;
        let reference = env.create_reference(constructor)?;
        self.objects_mut().insert(name.to_owned(), reference);
        Ok(self.object_constructor_function(name, env)?.unwrap())
    }

    fn object_prototype_or_create(&mut self, key: &str, env: Env) -> Result<JsObject> {
        let constructor = self.object_constructor_function_or_create(key, env)?;
        let constructor = constructor.coerce_to_object()?;
        let prototype: JsObject = constructor.get_named_property("prototype")?;
        Ok(prototype)
    }
}