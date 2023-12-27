use teo::prelude::response::header::readwrite::HeaderMap;

#[napi]
pub struct ReadWriteHeaderMap {
    pub(super) inner: HeaderMap
}

#[napi]
impl ReadWriteHeaderMap {

    #[napi]
    pub fn keys(&self) -> Vec<String> {
        self.inner.keys()
    }

    #[napi]
    pub fn len(&self) -> i64 {
        self.inner.len() as i64
    }

    #[napi(js_name = "containsKey")]
    pub fn contains_key(&self, key: String) -> bool {
        self.inner.contains_key(key)
    }

    #[napi(js_name = "get")]
    pub fn get(&self, key: String) -> Option<String> {
        self.inner.get(key)
    }

    #[napi(js_name = "set")]
    pub fn set(&self, key: String, value: String) {
        self.inner.set(key, value)
    }
}