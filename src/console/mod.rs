use napi::{Env, Result, JsObject, JsFunction, NapiRaw};

pub fn console_log<T>(env: Env, arg: T) -> Result<()> where T: NapiRaw {
    let global = env.get_global()?;
    let console: JsObject = global.get_named_property("console")?;
    let log: JsFunction = console.get_named_property("log")?;
    let _ = log.call(None, &[arg])?;
    Ok(())
}