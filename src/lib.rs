#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

pub mod value;

use napi::threadsafe_function::{ThreadsafeFunction, ErrorStrategy};
use napi::{Env, JsObject, JsString, JsFunction, Result};
use teo::core::app::{builder::AppBuilder, entrance::Entrance};
use teo::core::pipeline::items::function::validate::{ValidateResult, Validity};
use teo::core::teon::Value as TeoValue;
use to_mut::ToMut;
use value::{teo_value_to_js_unknown, WrappedTeoValue};

#[napi(js_name = "App")]
pub struct App {
    builder: AppBuilder
}

#[napi]
impl App {

    #[napi(constructor)]
    pub fn new(env: Env) -> Self {
        Self::with_cli(env, false)
    }

    #[napi(factory)]
    pub fn with_cli(env: Env, cli: bool) -> Self {
        let entrance = if cli { Entrance::CLI } else { Entrance::APP };
        let global = env.get_global().unwrap();
        let process: JsObject = global.get_named_property("process").unwrap();
        let version: JsString = process.get_named_property("version").unwrap();
        let version_str: String = version.into_utf8().unwrap().as_str().unwrap().to_owned();
        App { builder: AppBuilder::new_with_environment_version_and_entrance(teo::core::app::environment::EnvironmentVersion::NodeJS(version_str), entrance) }
    }

    #[napi]
    pub async fn run(&self) {
        let mut_builder = self.builder.to_mut();
        let teo_app = mut_builder.build().await;
        let _ = teo_app.run().await;
    }

    #[napi(ts_args_type = "callback: (input: any) => any")]
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

    #[napi(ts_args_type = "callback: (input: any) => bool | string | undefined | null")]
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

    #[napi]
    pub fn callback(&self, name: String, callback: JsFunction) -> Result<()> {
        Ok(())
    }

    #[napi]
    pub fn compare(&self, name: String, callback: JsFunction) -> Result<()> {
        Ok(())
    }
}
