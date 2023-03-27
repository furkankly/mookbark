use anyhow::Result;
use home::home_dir;
use std::path::PathBuf;
use tokio::{fs, io::AsyncWriteExt};

async fn get_session_file_path() -> PathBuf {
    // Store session_id in a file to use with other requests from the terminal app
    let mut file_path = PathBuf::new();
    let home = home_dir().unwrap();
    file_path.push(home);
    file_path.push(".mookbark/session");
    file_path
}

pub async fn write_to_session_file(cookie_val: &str) {
    let file_path = get_session_file_path().await;
    let prefix = file_path.parent().unwrap();
    fs::create_dir_all(prefix).await.unwrap();
    let mut file = fs::File::create(file_path).await.unwrap();
    let cookie_val = cookie_val.as_bytes();
    file.write_all(cookie_val)
        .await
        .expect("Failed to write to the session file");
}

pub async fn read_from_session_file() -> Result<String> {
    let file_path = get_session_file_path().await;
    let cookie_val = tokio::fs::read_to_string(file_path).await?;
    Ok(cookie_val)
}
