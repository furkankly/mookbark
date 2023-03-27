use axum::http::StatusCode;
use std::env;
use tower_http::{
    services::{ServeDir, ServeFile},
    set_status::SetStatus,
};

const DEFAULT_MOOKBARK_DIST_DIR: &str = "/opt/homebrew/share/mookbark/dist-cliAuth";

pub fn serve_token_page() -> ServeDir<SetStatus<ServeFile>> {
    let dist_dir =
        env::var("MOOKBARK_DIST_DIR").unwrap_or_else(|_| DEFAULT_MOOKBARK_DIST_DIR.to_string());

    ServeDir::new(&dist_dir).fallback(SetStatus::new(
        ServeFile::new(format!("{dist_dir}/cliAuth/index.html")),
        StatusCode::OK,
    ))
}
