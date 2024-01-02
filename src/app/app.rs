use teo::prelude::{App as TeoApp, Entrance, RuntimeVersion, transaction};
use napi::threadsafe_function::{ThreadsafeFunction, ErrorStrategy, ThreadSafeCallContext};
use napi::{Env, JsObject, JsString, JsFunction, Result, JsUnknown, Error};
use crate::dynamic::{synthesize_dynamic_nodejs_classes, js_ctx_object_from_teo_transaction_ctx};
use crate::namespace::Namespace;
use crate::object::promise_or_ignore::PromiseOrIgnore;
use crate::result::{IntoNodeJSResult, IntoTeoResult};

#[napi]
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
        // this load user's schema
        self.teo_app.prepare_for_run().into_nodejs_result()?;
        // synthesize dynamic running classes for Node.js
        synthesize_dynamic_nodejs_classes(&self.teo_app, env)?;
        // the CLI parsing and dispatch process
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

    #[napi(js_name = "mainNamespace", writable = false)]
    pub fn main_namespace(&'static self) -> Namespace {
        Namespace { teo_namespace: self.teo_app.main_namespace_mut() }
    }

    /// Run before server is started.
    #[napi(ts_args_type = "callback: (ctx: any) => void | Promise<void>")]
    pub fn setup(&self, callback: JsFunction) -> Result<()> {
        let tsfn: ThreadsafeFunction<transaction::Ctx, ErrorStrategy::Fatal> = callback.create_threadsafe_function(0, |ctx: ThreadSafeCallContext<transaction::Ctx>| {
            let js_ctx = js_ctx_object_from_teo_transaction_ctx(ctx.env, ctx.value.clone(), "")?;
            Ok(vec![js_ctx])
        })?;
        let tsfn_cloned = Box::leak(Box::new(tsfn));
        self.teo_app.setup(|ctx: transaction::Ctx| async {
            let promise_or_ignore: PromiseOrIgnore = tsfn_cloned.call_async(ctx).await.into_teo_result()?;
            promise_or_ignore.to_ignore().await.into_teo_result()?;
            Ok(())
        });
        Ok(())
    }

    /// Define a custom program.
    #[napi(ts_args_type = "name: string, callback: (ctx: any) => void | Promise<void>")]
    pub fn program(&self, name: String, callback: JsFunction) -> Result<()> {
        let tsfn: ThreadsafeFunction<transaction::Ctx, ErrorStrategy::Fatal> = callback.create_threadsafe_function(0, |ctx: ThreadSafeCallContext<transaction::Ctx>| {
            let js_ctx = js_ctx_object_from_teo_transaction_ctx(ctx.env, ctx.value.clone(), "")?;
            Ok(vec![js_ctx])
        })?;
        let tsfn_cloned = Box::leak(Box::new(tsfn));
        self.teo_app.program(name.as_str(), |ctx: transaction::Ctx| async {
            let promise_or_ignore: PromiseOrIgnore = tsfn_cloned.call_async(ctx).await.unwrap();
            Ok(promise_or_ignore.to_ignore().await.unwrap())
        });
        Ok(())
    }
}
