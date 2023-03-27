use mookbark::cli::commands::Parser;

#[async_std::main]
async fn main() {
    let server_mode = mookbark::cli::commands::Args::parse().server;

    match server_mode {
        Some(_server_mode) => {
            let _rocket = mookbark::rocket_mookbark_build().launch().await;
        }
        None => {
            let db = mookbark::cli::db::connect().await.unwrap();
            // Add the root container
            // TODO: move this to a more suitable place
            dbg!("Adding root node!");
            mookbark::cli::add_root::add_root(&db)
                .await
                .unwrap_or_else(|err| println!("{err}"));
            mookbark::cli::commands::parse_commands(&db)
                .await
                .unwrap_or_else(|err| {
                    eprintln!("{err}");
                });
        }
    }
}
