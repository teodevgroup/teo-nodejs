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

    #[napi(getter)]
    pub fn length(&self) -> u32 {
        self.original.len() as u32
    }
}