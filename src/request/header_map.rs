use teo::prelude::request::header::readonly::HeaderMap;

#[napi]
pub struct ReadOnlyHeaderMap {
    pub(super) inner: HeaderMap
}

#[napi]
impl ReadOnlyHeaderMap {

    #[napi]
    pub fn keys(&self) -> Vec<&str> {
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
    pub fn get(&self, key: String) -> Option<&str> {
        self.inner.get(key)
    }
}