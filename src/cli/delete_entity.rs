use crate::{
    cli::https_client::{get_https_client, SERVER_URL},
    is_valid_http_url,
};
use anyhow::Result;

pub async fn delete_entity(entity: &str) -> Result<()> {
    let https_client = get_https_client().await?;

    let entity_type = if is_valid_http_url(entity) {
        "bookmark"
    } else {
        "container"
    };
    let query = [(
        if entity_type == "bookmark" {
            "bookmark_url"
        } else {
            "container_name"
        },
        entity,
    )];
    let result = https_client
        .delete(format!("{}/{}", SERVER_URL, entity_type))
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
