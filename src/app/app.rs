use teo::prelude::{App as OriginalApp, Entrance, RuntimeVersion, transaction};
use napi::threadsafe_function::{ThreadsafeFunction, ErrorStrategy, ThreadSafeCallContext};
use napi::{Env, JsObject, JsString, JsFunction, Result, JsUnknown};
use crate::dynamic::{synthesize_dynamic_nodejs_classes, JSClassLookupMap};
use crate::namespace::Namespace;
use crate::object::promise_or_ignore::PromiseOrIgnore;

#[napi]
#[derive(Clone)]
pub struct App {
    pub(crate) original: OriginalApp,
}

/// A Teo app.
#[napi]
impl App {

    /// Create a Teo app.
    #[napi(constructor)]
    pub fn new(env: Env, argv: Option<Vec<String>>) -> Result<Self> {
        Self::with_cli(env, false, argv)
    }

    /// @internal
    #[napi(factory)]
    pub fn with_cli(env: Env, cli: bool, argv: Option<Vec<String>>) -> Result<Self> {
        let global = env.get_global()?;
        let process: JsObject = global.get_named_property("process")?;
        let version: JsString = process.get_named_property("version")?;
        let version_str: String = version.into_utf8()?.as_str()?.to_owned();
        let rust_argv = match argv {
            Some(argv) => argv,
            None => {
                let node_argv: JsObject = process.get_named_property("argv")?;
                let mut result = vec![];
                let len = node_argv.get_array_length()?;
                for i in 0..len {
                    let name: JsString = node_argv.get_element(i)?;
                    result.push(name.into_utf8()?.as_str()?.to_owned());
                }
                result
            }
        };
        let entrance = if cli { Entrance::CLI } else { Entrance::APP };
        let app = App { original: OriginalApp::new_with_entrance_and_runtime_version(Some(entrance), Some(RuntimeVersion::NodeJS(version_str)), Some(rust_argv))? };
        Ok(app)
    }

    /// @internal
    #[napi(js_name = "_prepare", ts_return_type="Promise<void>")]
    pub fn _prepare(&'static self, env: Env) -> Result<JsUnknown> {
        let js_function = env.create_function_from_closure("_prepare", |ctx| {
            let promise = ctx.env.execute_tokio_future((|| async {
                self.original.prepare_for_run().await?;
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
        synthesize_dynamic_nodejs_classes(&self.original, env)?;
        let promise: JsObject = env.execute_tokio_future((|| async {
        // the CLI parsing and dispatch process
        self.original.run_without_prepare().await?;
            Ok(0)
        })(), |env: &mut Env, _unknown: i32| {
            env.get_undefined()
        })?;
        Ok(promise)
    }

    /// Run before server is started.
    #[napi(ts_args_type = "callback: (ctx: any) => void | Promise<void>")]
    pub fn setup(&self, callback: JsFunction) -> Result<()> {
        let map = JSClassLookupMap::from_app_data(self.original.app_data());
        let threadsafe_callback: ThreadsafeFunction<transaction::Ctx, ErrorStrategy::Fatal> = callback.create_threadsafe_function(0, |ctx: ThreadSafeCallContext<transaction::Ctx>| {
            let js_ctx = map.teo_transaction_ctx_to_js_ctx_object(ctx.env, ctx.value.clone(), "")?;
            Ok(vec![js_ctx])
        })?;
        self.original.setup(move |ctx: transaction::Ctx| {
            let threadsafe_callback = threadsafe_callback.clone();
            async move {
                let promise_or_ignore: PromiseOrIgnore = threadsafe_callback.call_async(ctx).await?;
                Ok(promise_or_ignore.to_ignore().await?)    
            }
        });
        Ok(())
    }

    /// Define a custom program.
    #[napi(ts_args_type = "name: string, desc: string | undefined, callback: (ctx: any) => void | Promise<void>")]
    pub fn program(&'static self, name: String, desc: Option<String>, callback: JsFunction) -> Result<()> {
        let map = JSClassLookupMap::from_app_data(self.original.app_data());
        let threadsafe_callback: ThreadsafeFunction<transaction::Ctx, ErrorStrategy::Fatal> = callback.create_threadsafe_function(0, |ctx: ThreadSafeCallContext<transaction::Ctx>| {
            let js_ctx = map.teo_transaction_ctx_to_js_ctx_object(ctx.env, ctx.value.clone(), "")?;
            Ok(vec![js_ctx])
        })?;
        self.original.program(name.as_str(), desc, move |ctx: transaction::Ctx| {
            let threadsafe_callback = threadsafe_callback.clone();
            async move {
                let promise_or_ignore: PromiseOrIgnore = threadsafe_callback.call_async(ctx).await?;
                Ok(promise_or_ignore.to_ignore().await?)    
            }
        });
        Ok(())
    }

    #[napi(js_name = "mainNamespace", writable = false)]
    pub fn main_namespace(&'static self) -> Namespace {
        Namespace { 
            namespace_builder: self.original.main_namespace().clone(),
        }
    }
}
