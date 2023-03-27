pub mod cli;
pub mod terminal_app;
pub mod web_server;

pub fn is_valid_http_url(string: &str) -> bool {
    match string.parse::<axum::http::Uri>() {
        Ok(parsed) => {
            parsed.scheme() == Some(&axum::http::uri::Scheme::HTTP)
                || parsed.scheme() == Some(&axum::http::uri::Scheme::HTTPS)
        }
        Err(_) => false,
    }
}
