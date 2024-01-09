use teo::prelude::File as TeoFile;

/// File
/// File only represent input file in form request.
#[napi(js_name = "File")]
pub struct File {
    pub filepath: String,
    pub content_type: Option<String>,
    pub filename: String,
    pub filename_ext: Option<String>,
}

#[napi]
impl File { }

impl From<&TeoFile> for File {
    fn from(value: &TeoFile) -> Self {
        Self {
            filepath: value.filepath.clone(),
            content_type: value.content_type.clone(),
            filename: value.filename.clone(),
            filename_ext: value.filename_ext.clone(),
        }
    }
}

impl From<&File> for TeoFile {
    fn from(value: &File) -> Self {
        Self {
            filepath: value.filepath.clone(),
            content_type: value.content_type.clone(),
            filename: value.filename.clone(),
            filename_ext: value.filename_ext.clone(),
        }
    }
}