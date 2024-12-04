use napi::{Result, JsUnknown, Env};
use crate::pipeline::pipeline::Pipeline;
use teo::prelude::Pipeline as OriginalPipeline;

pub fn teo_pipeline_to_js_any(pipeline: &OriginalPipeline, env: &Env) -> Result<JsUnknown> {
    let instance = Pipeline::from(pipeline.clone()).into_instance(*env)?;
    Ok(instance.as_object(*env).into_unknown())
}