use crate::cli::https_client::{get_https_client, SERVER_URL};
use anyhow::Result;

pub async fn add_container(parent_container: Option<&str>, container: &str) -> Result<()> {
    let https_client = get_https_client().await?;

    let parent_container = parent_container.unwrap_or("root");
    let query = [
        ("parent_container_name", parent_container),
        ("container_name", container),
    ];
    let result = https_client
        .post(format!("{}/{}", SERVER_URL, "container"))
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
