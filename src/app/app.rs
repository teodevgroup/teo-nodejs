use teo::prelude::{App as TeoApp, Entrance, RuntimeVersion, transaction};
use napi::threadsafe_function::{ThreadsafeFunction, ErrorStrategy, ThreadSafeCallContext};
use napi::{Env, JsObject, JsString, JsFunction, Result, JsUnknown};
use crate::dynamic::{synthesize_dynamic_nodejs_classes, JSClassLookupMap};
use crate::namespace::Namespace;
use crate::object::promise_or_ignore::PromiseOrIgnore;

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
        let argv: JsObject = process.get_named_property("argv")?;
        let mut rust_argv = vec![];
        let len = argv.get_array_length()?;
        for i in 0..len {
            let name: JsString = argv.get_element(i)?;
            rust_argv.push(name.into_utf8()?.as_str()?.to_owned());
        }
        let entrance = if cli { Entrance::CLI } else { Entrance::APP };
        let app = App { teo_app: TeoApp::new_with_entrance_and_runtime_version(Some(entrance), Some(RuntimeVersion::NodeJS(version_str)), Some(rust_argv)).unwrap() };
        Ok(app)
    }

    /// @internal
    #[napi(js_name = "_prepare", ts_return_type="Promise<void>")]
    pub fn _prepare(&'static self, env: Env) -> Result<JsUnknown> {
        let js_function = env.create_function_from_closure("run", |ctx| {
            let promise = ctx.env.execute_tokio_future((|| async {
                self.teo_app.prepare_for_run().await?;
                Ok(0)
            })(), |env: &mut Env, _unknown: i32| {
                env.get_undefined()
            })?;
            Ok(promise)
        })?;
        let result: JsUnknown = js_function.call(None, &[env.get_undefined()?])?;
        Ok(result)
    }

    /// @internal
    #[napi(js_name = "_run", ts_return_type="Promise<void>")]
    pub fn _run(&'static self, env: Env) -> Result<JsObject> {
        // synthesize dynamic running classes for Node.js
        synthesize_dynamic_nodejs_classes(&self.teo_app, env)?;
        let promise: JsObject = env.execute_tokio_future((|| async {
        // the CLI parsing and dispatch process
        self.teo_app.run_without_prepare().await?;
            Ok(0)
        })(), |env: &mut Env, _unknown: i32| {
            env.get_undefined()
        })?;
        Ok(promise)
    }

    /// Run before server is started.
    #[napi(ts_args_type = "callback: (ctx: any) => void | Promise<void>")]
    pub fn setup(&'static self, callback: JsFunction) -> Result<()> {
        let map = JSClassLookupMap::from_app(&self.teo_app);
        let tsfn: ThreadsafeFunction<transaction::Ctx, ErrorStrategy::Fatal> = callback.create_threadsafe_function(0, |ctx: ThreadSafeCallContext<transaction::Ctx>| {
            let js_ctx = map.teo_transaction_ctx_to_js_ctx_object(ctx.env, ctx.value.clone(), "")?;
            Ok(vec![js_ctx])
        })?;
        let tsfn_cloned = Box::leak(Box::new(tsfn));
        self.teo_app.setup(|ctx: transaction::Ctx| async {
            let promise_or_ignore: PromiseOrIgnore = tsfn_cloned.call_async(ctx).await?;
            Ok(promise_or_ignore.to_ignore().await?)
        });
        Ok(())
    }

    /// Define a custom program.
    #[napi(ts_args_type = "name: string, desc: string | undefined, callback: (ctx: any) => void | Promise<void>")]
    pub fn program(&'static self, name: String, desc: Option<String>, callback: JsFunction) -> Result<()> {
        let map = JSClassLookupMap::from_app(&self.teo_app);
        let tsfn: ThreadsafeFunction<transaction::Ctx, ErrorStrategy::Fatal> = callback.create_threadsafe_function(0, |ctx: ThreadSafeCallContext<transaction::Ctx>| {
            let js_ctx = map.teo_transaction_ctx_to_js_ctx_object(ctx.env, ctx.value.clone(), "")?;
            Ok(vec![js_ctx])
        })?;
        let tsfn_cloned = Box::leak(Box::new(tsfn));
        self.teo_app.program(name.as_str(), desc, |ctx: transaction::Ctx| async {
            let promise_or_ignore: PromiseOrIgnore = tsfn_cloned.call_async(ctx).await?;
            Ok(promise_or_ignore.to_ignore().await?)
        });
        Ok(())
    }

    #[napi(js_name = "mainNamespace", writable = false)]
    pub fn main_namespace(&'static self) -> Namespace {
        Namespace { 
            namespace_builder: self.teo_app.main_namespace().clone(),
            app: &self.teo_app,
        }
    }
}
