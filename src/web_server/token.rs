use crate::web_server::{
    create_server::AppState,
    oauth::{DiscordUser, GoogleUser, OAuthUser},
    services,
};
use axum::{
    extract::{Extension, Query},
    http::StatusCode,
};
use nanoid::nanoid;
use oauth2::{basic::BasicClient, reqwest::async_http_client};
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, RedirectUrl, TokenResponse, TokenUrl,
};
use serde::Deserialize;
use std::{env, sync::Arc};
use tower_sessions::{Expiry, Session};

#[derive(Default)]
struct OAuthClient {
    pub client_id: String,
    pub client_secret: String,
    pub token_url: String,
    pub user_url: String,
}

#[derive(Debug, Deserialize)]
pub struct Params {
    redirect_uri: String,
    code: String,
    state: String,
}

pub async fn token(
    session: Session,
    Extension(app_state): Extension<Arc<AppState>>,
    Query(params): Query<Params>,
) -> Result<StatusCode, StatusCode> {
    let Params {
        redirect_uri,
        code,
        state: _state,
    } = params;

    let redirect_url = reqwest::Url::parse(redirect_uri.as_str()).unwrap();
    let oauth_provider = redirect_url
        .query_pairs()
        .find(|(key, _)| key == "oauth_provider")
        .map(|(_, value)| value)
        .unwrap()
        .to_string();

    let mut oauth_client: Option<OAuthClient> = None;

    if oauth_provider == "discord" {
        oauth_client = Some(OAuthClient {
            client_id: env::var("OAUTH_DISCORD_CLIENT_ID")
                .expect("OAUTH_DISCORD_CLIENT_ID env var not found"),
            client_secret: env::var("OAUTH_DISCORD_CLIENT_SECRET")
                .expect("OAUTH_DISCORD_CLIENT_SECRET env var not found"),
            token_url: env::var("OAUTH_DISCORD_TOKEN_URL")
                .expect("OAUTH_DISCORD_TOKEN_URL env var not found"),
            user_url: env::var("OAUTH_DISCORD_USER_URL")
                .expect("OAUTH_DISCORD_USER_URL env var not found"),
        });
    } else if oauth_provider == "google" {
        oauth_client = Some(OAuthClient {
            client_id: env::var("OAUTH_GOOGLE_CLIENT_ID")
                .expect("OAUTH_GOOGLE_CLIENT_ID env var not found"),
            client_secret: env::var("OAUTH_GOOGLE_CLIENT_SECRET")
                .expect("OAUTH_GOOGLE_CLIENT_SECRET env var not found"),
            token_url: env::var("OAUTH_GOOGLE_TOKEN_URL")
                .expect("OAUTH_GOOGLE_TOKEN_URL env var not found"),
            user_url: env::var("OAUTH_GOOGLE_USER_URL")
                .expect("OAUTH_GOOGLE_USER_URL env var not found"),
        });
    } else {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    // Create an OAuth2 client by specifying the client ID, client secret, authorization URL and
    // token URL.
    let client = BasicClient::new(
        ClientId::new(oauth_client.as_ref().unwrap().client_id.to_string()),
        Some(ClientSecret::new(
            oauth_client.as_ref().unwrap().client_secret.to_string(),
        )),
        // A dummy url as we don't generate an auth url in backend
        AuthUrl::new("https://auth".to_string()).expect("OAuth url parse error"),
        Some(
            TokenUrl::new(oauth_client.as_ref().unwrap().token_url.to_string())
                .expect("OAuth token url parse error"),
        ),
    )
    .set_redirect_uri(RedirectUrl::new(redirect_uri).expect("Redirect url parse error"));
    // Set the URL the user will be redirected to after the authorization process.

    // Now you can trade it for an access token.
    let token_result = client
        .exchange_code(AuthorizationCode::new(code))
        .request_async(async_http_client)
        .await
        .expect("Token request failed");
    session.set_expiry(Some(Expiry::OnInactivity(
        token_result
            .expires_in()
            .expect("Token doesn't expire")
            .try_into()
            .unwrap(),
    )));

    let client = reqwest::Client::new();
    let res = client
        .get(oauth_client.as_ref().unwrap().user_url.to_string())
        .bearer_auth(token_result.access_token().secret())
        .send()
        .await;

    let user = res
        .expect("user info request failed")
        .text()
        .await
        .expect("Failed getting user");

    if oauth_provider == "google" {
        let user: GoogleUser = serde_json::from_str(user.as_str()).expect("Failed parsing user");
        create_or_signin_user(session, app_state, oauth_provider, user.into()).await?;
    } else {
        let user: DiscordUser = serde_json::from_str(user.as_str()).expect("Failed parsing user");
        create_or_signin_user(session, app_state, oauth_provider, user.into()).await?;
    }

    Ok(StatusCode::OK)
}

async fn create_or_signin_user(
    session: Session,
    app_state: Arc<AppState>,
    oauth_provider: String,
    oauth_user: OAuthUser,
) -> Result<(), StatusCode> {
    let user = services::check_user(&app_state.db_conn, &oauth_provider, &oauth_user.id)
        .await
        .map_err(|err| {
            let err = err.to_string();
            tracing::error!(err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let mut user_id = nanoid!();
    match user {
        Some(user) => {
            user_id = user.user_id.clone();
        }
        None => {
            services::add_user(&app_state.db_conn, &oauth_provider, &user_id, oauth_user)
                .await
                .map_err(|err| {
                    let err = err.to_string();
                    tracing::error!(err);
                    StatusCode::INTERNAL_SERVER_ERROR
                })?;
        }
    }
    session.insert("user_id", user_id).map_err(|err| {
        let err = err.to_string();
        tracing::error!(err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(())
}
