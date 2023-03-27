use std::{error::Error, str::FromStr};

pub use clap::{CommandFactory, Parser, Subcommand};
use sea_orm::DatabaseConnection;

use crate::cli;

#[derive(Debug, Subcommand)]
pub enum AddSubcommand {
    /// Add a bookmark
    Bookmark {
        /// If doesn't exist, gets created and added to the topmost level
        /// containing the bookmark
        #[arg(short, long)]
        container: Option<String>,
        /// Gets added to the topmost level when --container is not specified
        #[arg(short, long)]
        bookmark: String,
    },
    /// Add a container
    Container {
        /// Set as "root" to add to the topmost level
        #[arg(short, long)]
        parent_container: String,
        #[arg(short, long)]
        container: String,
    },
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Login using your Discord account
    Login,
    /// Add a bookmark/container
    Add {
        #[command(subcommand)]
        add_command: AddSubcommand,
    },
    /// Delete a bookmark/container
    Delete {
        #[arg(short, long)]
        entity: String,
    },
    /// Get the bookmarks
    Get,
}

#[derive(Debug, Clone)]
pub struct Server;
impl FromStr for Server {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.eq("server") {
            let error = String::from("Incorrect argument while trying to run in server mode!");
            return Err(error);
        }
        Ok(Server)
    }
}

#[derive(Debug, Parser)]
pub struct Args {
    #[clap(hide(true))]
    pub server: Option<Server>,
    #[command(subcommand)]
    pub command: Option<Commands>,
}

pub async fn parse_commands(db: &DatabaseConnection) -> Result<(), Box<dyn Error>> {
    let command = Args::parse().command;
    match command {
        Some(command) => match command {
            Commands::Login => {
                cli::auth::login::login().await?;
                let _rocket =
                    async_std::task::spawn(crate::rocket_local_mookbark_build().launch()).await;
            }
            Commands::Add { add_command } => match add_command {
                AddSubcommand::Bookmark {
                    container,
                    bookmark,
                } => {
                    cli::add_bookmark::add_bookmark(&db, container.as_deref(), &bookmark).await?;
                }
                AddSubcommand::Container {
                    parent_container,
                    container,
                } => {
                    cli::add_container::add_container(&db, &parent_container, &container).await?;
                }
            },
            Commands::Delete { entity } => {
                cli::delete_entity::delete_entity(&db, &entity).await?;
            }
            Commands::Get => {
                let bookmarks_tree = cli::get_bookmarks::get_bookmarks(&db).await?;
                println!("{bookmarks_tree}");
            }
        },
        None => {
            let mut cmd = Args::command();
            cmd.print_help().expect("Error printing help!");
        }
    };
    return Ok(());
}
