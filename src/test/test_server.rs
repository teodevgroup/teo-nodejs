use napi::Result;
use crate::app::app::App;
use super::{TestRequest, TestResponse};

#[napi]
pub struct TestServer {
    server: teo::server::server::Server,
}

#[napi]
impl TestServer {
    #[napi(constructor)]
    pub fn new(app: &App) -> Self {
        Self { 
            server: teo::server::server::Server::new(app.teo_app.clone())
        }
    }

    #[napi]
    pub async fn setup(&self) -> Result<()> {
        Ok(self.server.setup_app_for_unit_test().await?)
    }

    #[napi]
    pub async fn reset(&self) -> Result<()> {
        Ok(self.server.reset_app_for_unit_test().await?)
    }

    #[napi]
    pub async fn process(&self, request: &TestRequest) -> Result<TestResponse> {
        let hyper_request = request.to_hyper_request();
        let response = self.server.process_test_request_with_hyper_request(hyper_request).await?;
        Ok(TestResponse::new(response))
    }
}