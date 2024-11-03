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
}