use dotenvy::dotenv;
use oauth2::basic::BasicClient;
use oauth2::{
    AuthUrl, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, RedirectUrl, Scope, TokenUrl,
};
use std::{env, error::Error};

pub async fn login() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let client_id = env::var("CLIENT_ID").expect("CLIENT_ID not found");
    let client_secret = env::var("CLIENT_SECRET").expect("CLIENT_SECRET not found");

    // Create an OAuth2 client by specifying the client ID, client secret, authorization URL and
    // token URL.
    let client = BasicClient::new(
        ClientId::new(client_id.to_string()),
        Some(ClientSecret::new(client_secret.to_string())),
        AuthUrl::new("https://discord.com/oauth2/authorize".to_string())?,
        Some(TokenUrl::new(
            "https://discord.com/api/oauth2/token".to_string(),
        )?),
    )
    // Set the URL the user will be redirected to after the authorization process.
    .set_redirect_uri(RedirectUrl::new(
        "https://localhost:5172/login".to_string(),
    )?);

    // Generate a PKCE challenge.
    // let (pkce_challenge, _pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    // Generate the full authorization URL.
    let (auth_url, _csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        // Set the desired scopes.
        .add_scope(Scope::new("identify".to_string()))
        // Set the PKCE code challenge.
        // .set_pkce_challenge(pkce_challenge)
        .url();

    // This is the URL you should redirect the user to, in order to trigger the authorization
    // process.
    println!("Browse to: {}", auth_url);

    return Ok(());
}
