use crate::cli::https_client::{get_https_client, SERVER_URL};
use anyhow::Result;
use termtree;

pub async fn get_bookmarks() -> Result<termtree::Tree<std::string::String>> {
    let https_client = get_https_client().await?;

    let result = https_client
        .get(format!("{}/bookmarks", SERVER_URL))
        .send()
        .await;
    match result {
        Ok(response) => match response.error_for_status() {
            Ok(response) => {
                let mut response = response.json::<termtree::Tree<String>>().await.unwrap();
                response.root = String::from("Mookbark");
                Ok(response)
            }
            Err(err) => Err(err.into()),
        },
        Err(err) => Err(err.into()),
    }
}
