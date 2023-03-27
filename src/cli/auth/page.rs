use rocket::{fs::NamedFile, get};
use std::path::{Path, PathBuf};

#[catch(404)]
pub async fn index() -> NamedFile {
    NamedFile::open(Path::new("dist/cliAuth/index.html"))
        .await
        .expect("index.html is not found")
}

#[get("/<file..>")]
pub async fn build_dir(file: PathBuf) -> NamedFile {
    let file = NamedFile::open(Path::new("dist/").join(file)).await;
    match file {
        Ok(file) => file,
        _ => NamedFile::open(Path::new("dist/cliAuth/index.html"))
            .await
            .expect("index.html is not found"),
    }
}
