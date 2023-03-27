use axum::http::StatusCode;
use tower_http::{
    services::{ServeDir, ServeFile},
    set_status::SetStatus,
};

pub fn serve_web_app() -> ServeDir<SetStatus<ServeFile>> {
    ServeDir::new("client/apps/web/dist/").fallback(SetStatus::new(
        ServeFile::new("client/apps/web/dist/index.html"),
        StatusCode::OK,
    ))
}
