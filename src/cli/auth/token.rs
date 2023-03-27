use crate::cli::{
    auth::{create_server::SHUTDOWN_TX, file::write_to_session_file},
    https_client::SERVER_URL,
};
use axum::extract::Query;
use axum::http::StatusCode;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Params {
    redirect_uri: String,
    code: String,
    state: String,
}

pub async fn token(Query(params): Query<Params>) -> Result<StatusCode, StatusCode> {
    let Params {
        redirect_uri,
        code,
        state,
    } = params;

    let redirect_uri = reqwest::Url::parse_with_params(
        &format!("{}/token", SERVER_URL),
        &[
            ("redirect_uri", redirect_uri),
            ("code", code),
            ("state", state),
        ],
    )
    .expect("Failed to build URL");
    let client = reqwest::Client::new();
    let result = client
        .post(redirect_uri)
        .send()
        .await
        .map_err(|_err| StatusCode::INTERNAL_SERVER_ERROR);

    match result {
        Ok(response) => match response.error_for_status() {
            Ok(response) => {
                if let Some(cookie) = response.cookies().find(|c| c.name() == "id") {
                    let cookie_val = cookie.value();
                    write_to_session_file(cookie_val).await;
                } else {
                    return Err(StatusCode::UNAUTHORIZED);
                }
                // Attempt to send a shutdown signal, if one hasn't already been sent
                if let Some(tx) = SHUTDOWN_TX.lock().await.take() {
                    let _ = tx.send(());
                }

                Ok(StatusCode::OK)
            }
            Err(_err) => {
                println!("Authentication failed! Please report this and try again.");
                Err(StatusCode::UNAUTHORIZED)
            }
        },
        Err(err) => {
            println!("Authentication failed! The servers seem to be down. Please try again.");
            Err(err)
        }
    }
}
