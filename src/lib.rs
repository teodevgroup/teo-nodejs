use std::sync::Arc;
use neon::prelude::*;
use teo::core::app::builder::AppBuilder as TeoAppBuilder;
use teo::core::app::entrance::Entrance;
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
        let cli_mode = cx.argument_opt(0);

        let app_builder = AppBuilder {
            app_builder: Arc::new(if let Some(cli_mode) = cli_mode {
                let cli_mode_bool: Handle<JsBoolean> = cli_mode.downcast(&mut cx).unwrap_or(cx.boolean(false));
                if cli_mode_bool.value(&mut cx) {
                    TeoAppBuilder::new_with_environment_version_and_entrance(EnvironmentVersion::NodeJS(version_str), Entrance::CLI)
                } else {
                    TeoAppBuilder::new_with_environment_version(EnvironmentVersion::NodeJS(version_str))
                }
            } else {
                TeoAppBuilder::new_with_environment_version(EnvironmentVersion::NodeJS(version_str))
            })
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
            let app_builder_ref = app_builder.as_ref();
            let app_builder_ref_mut = app_builder_ref.to_mut();
            let app = app_builder_ref_mut.build().await;
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
    cx.export_function("appBuilderBuild", AppBuilder::build)?;
    Ok(())
}
