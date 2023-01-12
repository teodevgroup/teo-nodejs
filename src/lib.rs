use std::sync::Arc;
use neon::prelude::*;
use teo::core::app::builder::AppBuilder as TeoAppBuilder;
use teo::core::app::environment::EnvironmentVersion;
use to_mut::ToMut;
use tokio::runtime::Runtime;
use once_cell::sync::OnceCell;

fn get_runtime<'a, C: Context<'a>>(cx: &mut C) -> NeonResult<&'static Runtime> {
    static RUNTIME: OnceCell<Runtime> = OnceCell::new();
    RUNTIME.get_or_try_init(|| Runtime::new().or_else(|err| cx.throw_error(err.to_string())))
}

pub struct AppBuilder {
    app_builder: Arc<TeoAppBuilder>,
}

impl AppBuilder {
    fn js_new(mut cx: FunctionContext) -> JsResult<JsBox<AppBuilder>> {
        let process: Handle<JsObject> = cx.global().get(&mut cx, "process")?;
        let version: Handle<JsString> = process.get(&mut cx, "version")?;
        let version_str = version.value(&mut cx);
        let app_builder = AppBuilder {
            app_builder: Arc::new(TeoAppBuilder::new_with_environment_version(EnvironmentVersion::NodeJS(version_str)))
        };
        Ok(cx.boxed(app_builder))
    }

    fn build(mut cx: FunctionContext) -> JsResult<JsPromise> {
        let runtime = get_runtime(&mut cx)?;
        let channel = cx.channel();
        let this = cx.this().downcast_or_throw::<JsBox<AppBuilder>, _>(&mut cx)?;
        let app_builder = this.app_builder.clone();
        let (deferred, promise) = cx.promise();
        runtime.spawn(async move {
            let app = app_builder.build().await;
            app.run().await;
            deferred.settle_with(&channel, move |mut cx| {
                Ok(cx.undefined())
            });
        });
        Ok(promise)
    }
}

impl Finalize for AppBuilder { }

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("createAppBuilder", AppBuilder::js_new)?;
    cx.export_function("appBuilderLoad", AppBuilder::load)?;
    cx.export_function("appBuilderBuild", AppBuilder::build)?;
    Ok(())
}
