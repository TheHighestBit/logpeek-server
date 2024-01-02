use axum::{response::{IntoResponse, Response}, http::{Uri, header, StatusCode}};
use log::{trace, info};
use rust_embed::RustEmbed;

pub async fn index_handler() -> impl IntoResponse {
    trace!("Index page requested");
    static_handler("/index.html".parse::<Uri>().unwrap()).await
}

pub async fn static_handler(uri: Uri) -> impl IntoResponse {
    trace!("Static file requested: {}", uri.path());
    let mut path = uri.path().trim_start_matches('/').to_string();
    if path.starts_with("dist/") {
        path = path.replace("dist/", "");
    }

    StaticFile(path)
}

#[derive(RustEmbed)]
#[folder = "src/dist/"]
struct Asset;

pub struct StaticFile<T>(pub T);

impl<T> IntoResponse for StaticFile<T>
where
  T: Into<String>,
{
  fn into_response(self) -> Response {
    let path = self.0.into();
    //info!("Serving static file: {}", path);

    match Asset::get(path.as_str()) {
      Some(content) => {
        let mime = mime_guess::from_path(path).first_or_octet_stream();
        ([(header::CONTENT_TYPE, mime.as_ref())], content.data).into_response()
      }
      None => (StatusCode::NOT_FOUND, "404 Not Found").into_response(),
    }
  }
}