use napi::{Env, Error, JsObject, JsUnknown, Result};
use teo::prelude::{model, transaction};

use super::fetch::FetchDynamicClasses;

pub trait QueryDynamicClasses: FetchDynamicClasses {

    fn teo_model_ctx_to_js_model_class_object(&self, env: Env, model_ctx: model::Ctx, name: &str) -> Result<JsObject> {
        let Some(class_prototype) = self.class_prototype(name, env)? else {
            return Err(Error::from_reason("Class prototype not found"));
        };
        let mut js_object = env.create_object()?;
        js_object.set_named_property("__proto__", class_prototype)?;
        env.wrap(&mut js_object, model_ctx)?;
        Ok(js_object)
    }

    fn teo_model_object_to_js_model_object_object(&self, env: Env, teo_model_object: model::Object) -> Result<JsObject> {
        let Some(object_prototype) = self.object_prototype(&teo_model_object.model().path().join("."), env)? else {
            return Err(Error::from_reason("Object prototype not found"));
        };
        let mut js_object = env.create_object()?;
        js_object.set_named_property("__proto__", object_prototype)?;
        env.wrap(&mut js_object, teo_model_object)?;
        Ok(js_object)
    }

    fn teo_optional_model_object_to_js_optional_model_object_object(&self, env: Env, teo_model_object: Option<model::Object>) -> Result<JsUnknown> {
        Ok(match teo_model_object {
            Some(teo_model_object) => self.teo_model_object_to_js_model_object_object(env, teo_model_object)?.into_unknown(),
            None => env.get_undefined()?.into_unknown(),
        })
    }

    fn teo_transaction_ctx_to_js_ctx_object(&self, env: Env, transaction_ctx: transaction::Ctx, name: &str) -> Result<JsObject> {
        let Some(ctx_prototype) = self.ctx_prototype(name, env)? else {
            return Err(Error::from_reason("Ctx prototype not found"));
        };
        let mut js_object = env.create_object()?;
        js_object.set_named_property("__proto__", ctx_prototype)?;
        env.wrap(&mut js_object, transaction_ctx)?;
        Ok(js_object)
    }
}