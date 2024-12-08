use teo::prelude::headers::Headers as OriginalHeaders;
use napi::Result;

#[derive(Clone)]
#[napi]
pub struct Headers {
    original: OriginalHeaders
}

impl From<OriginalHeaders> for Headers {
    fn from(original: OriginalHeaders) -> Self {
        Self { original }
    }
}

impl Headers {
    pub(crate) fn original(&self) -> &OriginalHeaders {
        &self.original
    }
}

#[napi]
impl Headers {

    #[napi]
    pub fn append(&self, key: String, value: String) -> Result<()> {
        Ok(self.original.append(key, value)?)
    }

    #[napi]
    pub fn set(&self, key: String, value: String) -> Result<()> {
        Ok(self.original.insert(key, value)?)
    }

    #[napi]
    pub fn get(&self, key: String) -> Result<Option<String>> {
        Ok(self.original.get(key)?)
    }

    #[napi]
    pub fn get_all(&self, key: String) -> Result<Vec<String>> {
        Ok(self.original.get_all(key)?)
    }

    #[napi]
    pub fn delete(&self, key: String) {
        self.original.remove(key)
    }

    #[napi]
    pub fn keys(&self) -> Vec<String> {
        self.original.keys()
    }

    #[napi]
    pub fn values(&self) -> Vec<String> {
        self.original.values()
    }

    #[napi(getter)]
    pub fn length(&self) -> i64 {
        self.original.len() as i64
    }

    #[napi]
    pub fn has(&self, key: String) -> bool {
        self.original.contains_key(key)
    }
}