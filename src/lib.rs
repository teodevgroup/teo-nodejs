#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use napi::{Env, JsObject, JsString};
use teo::core::app::{builder::AppBuilder, entrance::Entrance};
use to_mut::ToMut;

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
}
