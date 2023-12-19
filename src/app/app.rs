use teo::prelude::{App as TeoApp, app, Entrance, RuntimeVersion};
use napi::threadsafe_function::{ThreadsafeFunction, ErrorStrategy, ThreadSafeCallContext};
use napi::{Env, JsObject, JsString, JsFunction, Result, JsUnknown, Error, JsSymbol, CallContext, Property, ValueType, JsUndefined};
use crate::result::IntoNodeJSResult;

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
        let global = env.get_global()?;
        let process: JsObject = global.get_named_property("process")?;
        let version: JsString = process.get_named_property("version")?;
        let version_str: String = version.into_utf8()?.as_str()?.to_owned();
        let entrance = if cli { Entrance::CLI } else { Entrance::APP };
        let app = App { teo_app: TeoApp::new_with_entrance_and_runtime_version(Some(entrance), Some(RuntimeVersion::NodeJS(version_str))).unwrap() };
        Ok(app)
    }

    /// Run this app.
    #[napi(ts_return_type="Promise<void>")]
    pub fn run(&self, env: Env) -> Result<JsUnknown> {
        self.teo_app.prepare_for_run().into_nodejs_result()?;
        let static_self: &'static App = unsafe { &*(self as * const App) };
        let js_function = env.create_function_from_closure("run", |ctx| {
            let promise = ctx.env.execute_tokio_future((|| async {
                static_self.teo_app.run_without_prepare().await.into_nodejs_result()?;
                Ok(0)
            })(), |env: &mut Env, _unknown: i32| {
                env.get_undefined()
            })?;
            Ok(promise)
        })?;
        let result: JsUnknown = js_function.call(None, &[env.get_undefined()?])?;
        Ok(result)
    }
}
