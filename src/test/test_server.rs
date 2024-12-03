use napi::{Env, Result};
use crate::{app::app::App, dynamic::synthesize_dynamic_nodejs_classes};
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
            server: teo::server::server::Server::new(app.original.clone())
        }
    }

    /// @internal
    #[napi(js_name = "_setup_0")]
    pub async fn _setup_0(&self) -> Result<()> {
        Ok(self.server.setup_app_for_unit_test().await?)
    }

    /// @internal
    #[napi(js_name = "_setup_1")]
    pub fn _setup_1(&self, env: Env) -> Result<()> {
        synthesize_dynamic_nodejs_classes(&self.server.app, env)
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