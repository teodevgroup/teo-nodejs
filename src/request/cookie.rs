#[napi(js_name = "Cookie")]
pub struct Cookie {
    pub(crate) inner: teo::prelude::request::Cookie<'static>,
}

#[napi]
impl Cookie {

}
