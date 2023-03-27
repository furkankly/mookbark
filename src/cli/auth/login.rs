use anyhow::{Error, Result};
use oauth2::basic::BasicClient;
use oauth2::{
    AuthUrl,
    ClientId,
    // ClientSecret,
    CsrfToken,
    // PkceCodeChallenge,
    RedirectUrl,
    Scope,
};
use tokio::sync::oneshot::Receiver;

#[derive(Default)]
struct OAuthClient<'a> {
    pub client_id: String,
    pub auth_url: String,
    pub scopes: Vec<&'a str>,
}

// its ok for a cli app to expose client secrets
pub async fn login(oauth_provider: String, rx: Receiver<String>) -> Result<()> {
    let mut oauth_client: Option<OAuthClient> = None;

    if oauth_provider == "discord" {
        oauth_client = Some(OAuthClient {
            client_id: String::from("1062682150912151602"),
            auth_url: String::from("https://discord.com/oauth2/authorize"),
            scopes: vec!["identify"],
        });
    } else if oauth_provider == "google" {
        oauth_client = Some(OAuthClient {
            client_id: String::from(
                "690751686191-avavjqur4or8oi0ilqa4j6qjf8f747ju.apps.googleusercontent.com",
            ),
            auth_url: String::from("https://accounts.google.com/o/oauth2/v2/auth"),
            scopes: vec!["email", "profile"],
        });
    } else {
        return Err(Error::msg("Not a valid oauth provider"));
    }

    let client_id = oauth_client.as_ref().unwrap().client_id.to_string();
    let auth_url = oauth_client.as_ref().unwrap().auth_url.to_string();

    let redirect_uri = rx.await.unwrap();
    // Create an OAuth2 client by specifying the client ID, client secret, authorization URL and
    // token URL.
    let client = BasicClient::new(
        ClientId::new(client_id),
        None,
        AuthUrl::new(auth_url)?,
        None,
    )
    // Set the URL the user will be redirected to after the authorization process.
    .set_redirect_uri(RedirectUrl::new(
        format!(
            "http://{}/login?oauth_provider={}",
            redirect_uri, oauth_provider
        )
        .to_string(),
    )?);

    // Generate a PKCE challenge.
    // let (pkce_challenge, _pkce_verifier) = PkceCodeChallenge::new_random_sha256();
    let scopes = oauth_client
        .as_ref()
        .unwrap()
        .scopes
        .iter()
        .map(|scope| Scope::new(scope.to_string()));
    // Generate the full authorization URL.
    let (auth_url, _csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        // Set the desired scopes.
        .add_scopes(scopes)
        // Set the PKCE code challenge.
        // .set_pkce_challenge(pkce_challenge)
        .url();

    // This is the URL you should redirect the user to, in order to trigger the authorization
    // process.
    let _ = webbrowser::open(auth_url.as_ref());
    println!("Browse to: {}", auth_url);

    Ok(())
}
