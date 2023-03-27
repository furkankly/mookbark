use crate::cli::auth::file::read_from_session_file;
use anyhow::Result;
use crossterm::style::Stylize;

pub const SERVER_URL: &str = "https://mookbark.run/api";

pub async fn get_https_client() -> Result<reqwest::Client> {
    let url = reqwest::Url::parse(SERVER_URL).unwrap();
    // Load an existing set of cookies, serialized as json
    let cookie_store = {
        let cookie_val = read_from_session_file().await.map_err(|_err| {
            anyhow::Error::msg(format!(
                "Please login to {} by running {} first.",
                "Mookbark".dark_yellow().bold(),
                "mookbark login".italic().bold()
            ))
        })?;
        let cookie = reqwest_cookie_store::RawCookie::build("id", cookie_val.trim())
            .domain(url.domain().unwrap())
            .path("/")
            .secure(true)
            .http_only(true)
            .finish();
        let mut store = reqwest_cookie_store::CookieStore::new(None);
        store.insert_raw(&cookie, &url).unwrap();
        store
    };
    let cookie_store = reqwest_cookie_store::CookieStoreMutex::new(cookie_store);
    let cookie_store = std::sync::Arc::new(cookie_store);

    // Build a `reqwest` Client, providing the deserialized store
    Ok(reqwest::Client::builder()
        .cookie_provider(std::sync::Arc::clone(&cookie_store))
        .build()
        .unwrap())
}
