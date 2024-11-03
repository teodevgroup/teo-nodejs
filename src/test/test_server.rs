use napi::Result;

use crate::app::app::App;

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
    pub async fn reset_data(&self) -> Result<()> {
        Ok(self.server.reset_app_for_unit_test().await?)
    }
}