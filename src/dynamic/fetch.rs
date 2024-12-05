use std::collections::BTreeMap;
use napi::{Env, JsFunction, JsObject, Result};

pub trait FetchDynamicClasses {

    fn ctxs(&self) -> &BTreeMap<String, napi::Ref<()>>;

    fn classes(&self) -> &BTreeMap<String, napi::Ref<()>>;

    fn objects(&self) -> &BTreeMap<String, napi::Ref<()>>;

    fn ctx_constructor_function(&self, key: &str, env: Env) -> Result<Option<JsFunction>> {
        match self.ctxs().get(key).map(|r| {
            let constructor: Result<JsFunction> = env.get_reference_value(r);
            constructor
        }) {
            None => Ok(None),
            Some(result) => match result {
                Ok(constructor) => Ok(Some(constructor)),
                Err(e) => Err(e),
            },
        }
    }

    fn ctx_constructor_object(&self, key: &str, env: Env) -> Result<Option<JsObject>> {
        let constructor = self.ctx_constructor_function(key, env)?;
        match constructor {
            None => Ok(None),
            Some(constructor) => {
                let constructor = constructor.coerce_to_object()?;
                Ok(Some(constructor))
            }
        }
    }

    fn ctx_prototype(&self, key: &str, env: Env) -> Result<Option<JsObject>> {
        let constructor = self.ctx_constructor_object(key, env)?;
        match constructor {
            None => Ok(None),
            Some(constructor) => {
                let prototype: JsObject = constructor.get_named_property("prototype")?;
                Ok(Some(prototype))
            }
        }
    }

    fn class_constructor_function(&self, key: &str, env: Env) -> Result<Option<JsFunction>> {
        match self.classes().get(key).map(|r| {
            let constructor: Result<JsFunction> = env.get_reference_value(r);
            constructor
        }) {
            None => Ok(None),
            Some(result) => match result {
                Ok(constructor) => Ok(Some(constructor)),
                Err(e) => Err(e),
            },
        }
    }

    fn class_constructor_object(&self, key: &str, env: Env) -> Result<Option<JsObject>> {
        let constructor = self.class_constructor_function(key, env)?;
        match constructor {
            None => Ok(None),
            Some(constructor) => {
                let constructor = constructor.coerce_to_object()?;
                Ok(Some(constructor))
            }
        }
    }

    fn class_prototype(&self, key: &str, env: Env) -> Result<Option<JsObject>> {
        let constructor = self.class_constructor_object(key, env)?;
        match constructor {
            None => Ok(None),
            Some(constructor) => {
                let prototype: JsObject = constructor.get_named_property("prototype")?;
                Ok(Some(prototype))
            }
        }
    }

    fn object_constructor_function(&self, key: &str, env: Env) -> Result<Option<JsFunction>> {
        match self.objects().get(key).map(|r| {
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

    fn object_constructor_object(&self, key: &str, env: Env) -> Result<Option<JsObject>> {
        let constructor = self.object_constructor_function(key, env)?;
        match constructor {
            None => Ok(None),
            Some(constructor) => {
                let constructor = constructor.coerce_to_object()?;
                Ok(Some(constructor))
            }
        }
    }

    fn object_prototype(&self, key: &str, env: Env) -> Result<Option<JsObject>> {
        let constructor = self.object_constructor_object(key, env)?;
        match constructor {
            None => Ok(None),
            Some(constructor) => {
                let prototype: JsObject = constructor.get_named_property("prototype")?;
                Ok(Some(prototype))
            }
        }
    }
}