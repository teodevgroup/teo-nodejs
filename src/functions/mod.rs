use teo::server::static_files::serve_static_files as teo_serve_static_files;
use napi::Result;
use crate::response::Response;

#[napi]
pub fn serve_static_files(base: String, path: String) -> Result<Response> {
    let teo_response = teo_serve_static_files(base, path)?;
    Ok(Response { teo_response })
}