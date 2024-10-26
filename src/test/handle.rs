use actix_web::test;
use teo::{app::App as TeoApp, prelude::request, test::{purge_and_seed::purge_and_seed, server::{make_actix_app_without_prepare, prepare_app_for_test}}};
use crate::app::app::App;
use napi::{Env, JsObject, Result};
use super::test_request::TestRequest;

#[napi]
pub struct Handle {
    app: TeoApp,
}

#[napi]
impl Handle {

    #[napi(constructor)]
    pub fn new(app: &App) -> Self {
        Self { app: app.teo_app.clone() }
    }

    #[napi]
    pub fn setup(&self) {

    }

    #[napi]
    pub async fn purge_and_seed(&self) -> Result<()> {
        Ok(purge_and_seed(&self.app).await?)
    }

    #[napi]
    pub async fn prepare_app_for_test(&self) -> Result<()> {
        Ok(prepare_app_for_test(&self.app).await?)
    }

    // #[napi]
    // pub async fn call_and_read_response(&self, request: &TestRequest) -> Result<()> {
    //     let service = test::init_service(make_actix_app_without_prepare(&self.app).await?).await;
    //     let test_request = request.to_actix_test_request().to_request();
    //     let response = test::call_service(&service, test_request).await;
    //     Ok(())
    // }

    // #[napi]
    // pub async fn call_and_read_json_body(&self, request: &TestRequest) -> Result<()> {
    //     let service = test::init_service(make_actix_app_without_prepare(&self.app).await?).await;
    //     let test_request = request.to_actix_test_request().to_request();
    //     let body: serde_json::Value = test::call_and_read_body_json(&service, test_request).await;
    //     Ok(())
    // }

    // #[napi]
    // pub async fn call_and_read_string_body(&self, request: &TestRequest) -> Result<()> {
    //     let service = test::init_service(make_actix_app_without_prepare(&self.app).await?).await;
    //     let test_request = request.to_actix_test_request().to_request();
    //     let body = test::call_and_read_body(&service, test_request).await;
    //     // body.
    //     Ok(())
    // }
}