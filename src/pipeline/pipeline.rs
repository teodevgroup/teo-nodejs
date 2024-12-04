use teo::prelude::Pipeline as OriginalPipeline;

#[napi]
pub struct Pipeline {
    pub(crate) original: OriginalPipeline
}

impl From<OriginalPipeline> for Pipeline {
    fn from(value: OriginalPipeline) -> Self {
        Self { original: value }
    }
}

#[napi]
impl Pipeline {

}