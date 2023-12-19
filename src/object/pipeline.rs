use napi::{Env, JsUnknown, Result};
use teo::prelude::Pipeline as TeoPipeline;

#[napi(js_name = "Pipeline")]
pub struct Pipeline {
    pub(crate) value: TeoPipeline
}

pub fn teo_pipeline_to_js_any(pipeline: &TeoPipeline, env: &Env) -> Result<JsUnknown> {
    let instance = Pipeline { value: pipeline.clone() }.into_instance(*env)?;
    Ok(instance.as_object(*env).into_unknown())
}