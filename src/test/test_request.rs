// use std::str::FromStr;

// use actix_web::http::Method;
// use napi::{JsObject, Result};

// #[napi]
// pub struct TestRequest {
//     method: String,
//     path: String,
//     body: String,
//     headers: Vec<(String, String)>,
// }

// #[napi]
// impl TestRequest {

//     #[napi(constructor, ts_args_type = "desc: { method?: string, path?: string, body?: string }")]
//     pub fn new(desc: JsObject) -> Result<Self> {
//         let method: Option<String> = desc.get_named_property("method")?;
//         let path: Option<String> = desc.get_named_property("path")?;
//         let body: Option<String> = desc.get_named_property("body")?;
//         Ok(Self {
//             method: method.unwrap_or("POST".to_owned()),
//             path: path.unwrap_or("/".to_owned()),
//             body: body.unwrap_or("".to_owned()),
//             headers: vec![],
//         })
//     }

//     #[napi]
//     pub fn insert_header(&mut self, key: String, value: String) -> Result<()> {
//         self.headers.push((key, value));
//         Ok(())
//     }

//     pub(crate) fn to_actix_test_request(&self) -> actix_web::test::TestRequest {
//         let mut request = actix_web::test::TestRequest::with_uri(&self.path);
//         request = request.method(Method::from_str(self.method.as_str()).unwrap());
//         request = request.set_payload(self.body.clone());
//         for (key, value) in &self.headers {
//             request = request.append_header((key.as_str(), value.as_str()));
//         }
//         request
//     }
// }