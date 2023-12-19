use teo::prelude::{App as TeoApp};
use napi::threadsafe_function::{ThreadsafeFunction, ErrorStrategy, ThreadSafeCallContext};
use napi::{Env, JsObject, JsString, JsFunction, Result, JsUnknown, Error, JsSymbol, CallContext, Property, ValueType};


#[napi(js_name = "App")]
pub struct App {
    teo_app: TeoApp,
}

/// A Teo app.
#[napi]
impl crate::app::new_app::App {

    /// Create a Teo app.
    #[napi(constructor)]
    pub fn new(env: Env) -> napi::Result<Self> {
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
        let app = crate::app::app::App { teo_app: TeoApp::new().unwrap() };
        let app_ctx = AppCtx::get().unwrap();
        app_ctx.set_entrance(entrance).into_nodejs_result()?;
        app_ctx.set_program(Program::NodeJS(version_str)).into_nodejs_result()?;
        Ok(app)
    }
}