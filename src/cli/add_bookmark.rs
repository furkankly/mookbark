use crate::cli::https_client::{get_https_client, SERVER_URL};
use anyhow::Result;

pub async fn add_bookmark(container: Option<&str>, url: &str) -> Result<()> {
    let https_client = get_https_client().await?;

    let container = container.unwrap_or("root");
    let query = [("container_name", container), ("bookmark_url", url)];
    let result = https_client
        .post(format!("{}/{}", SERVER_URL, "bookmark"))
        .query(&query)
        .send()
        .await;
    match result {
        Ok(response) => match response.error_for_status() {
            Ok(_response) => Ok(()),
            Err(err) => Err(err.into()),
        },
        Err(err) => Err(err.into()),
    }
}
