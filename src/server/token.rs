use dotenvy::dotenv;
use oauth2::basic::{BasicClient, BasicTokenType};
use oauth2::reqwest::async_http_client;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, EmptyExtraTokenFields, RedirectUrl,
    StandardTokenResponse, TokenResponse, TokenUrl,
};
use rocket::http::Status;
use rocket::serde::json::Json;
use std::env;

#[post("/token?<redirect_uri>&<code>")]
pub async fn token(
    redirect_uri: String,
    code: String,
) -> Result<Json<StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>>, Status> {
    dotenv().ok();
    let client_id = env::var("CLIENT_ID").expect("CLIENT_ID not found");
    let client_secret = env::var("CLIENT_SECRET").expect("CLIENT_SECRET not found");

    // Create an OAuth2 client by specifying the client ID, client secret, authorization URL and
    // token URL.
    let client = BasicClient::new(
        ClientId::new(client_id.to_string()),
        Some(ClientSecret::new(client_secret.to_string())),
        // A dummy url as we don't generate an auth url in backend
        AuthUrl::new("https://auth".to_string()).unwrap(),
        Some(TokenUrl::new("https://discord.com/api/oauth2/token".to_string()).unwrap()),
    )
    .set_redirect_uri(RedirectUrl::new(redirect_uri).unwrap());
    // Set the URL the user will be redirected to after the authorization process.

    // Now you can trade it for an access token.
    let token_result = client
        .exchange_code(AuthorizationCode::new(code))
        .request_async(async_http_client)
        .await
        .unwrap();

    let client = reqwest::Client::new();
    let res = client
        .get("https://discord.com/api/users/@me")
        .bearer_auth(token_result.access_token().secret())
        .send()
        .await;

    println!("User information: {:#?}", res.unwrap().text().await);

    return Ok(Json(token_result));
    // Unwrapping token_result will either produce a Token or a RequestTokenError.
}
