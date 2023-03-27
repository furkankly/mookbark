use anyhow::Result;
use dotenvy::dotenv;
use mookbark::cli::commands::Parser;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    if env::var_os("CARGO_PKG_NAME").is_some() {
        dotenv().expect(".env file not found");
    }
    let server_mode = mookbark::cli::commands::Args::parse().server;
    match server_mode {
        Some(_) => {
            mookbark::web_server::create_server::create_server().await;
        }
        None => mookbark::cli::commands::parse_commands().await.unwrap(),
    }
    Ok(())
}
