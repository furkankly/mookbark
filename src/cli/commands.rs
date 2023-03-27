use crate::cli;
use crate::terminal_app;
use anyhow::Result;
pub use clap::{Parser, Subcommand};
use futures::future::join;
use std::str::FromStr;
// use tokio_scoped::Scope;

#[derive(Debug, Subcommand)]
pub enum AddSubcommand {
    /// Add a bookmark
    Bookmark {
        /// A container
        #[arg(short, long)]
        container: Option<String>,
        /// A bookmark, Added to the topmost level when --container (-c) is not specified
        #[arg(short, long)]
        bookmark: String,
    },
    /// Add a container
    Container {
        #[arg(short, long)]
        /// Parent container
        parent_container: Option<String>,
        /// A container, Added to the topmost level when --parent-container (-p) is not specified
        #[arg(short, long)]
        container: String,
    },
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Login to Mookbark using one of the supported OAuth providers
    Login {
        #[arg(short, long)]
        /// 'google' or 'discord'.
        /// Nevermind 'discord' OAuth doesn't support RFC8252 fully :(
        oauth_provider: String,
    },
    /// Add a bookmark/container
    Add {
        #[command(subcommand)]
        add_command: AddSubcommand,
    },
    /// Delete a bookmark/container
    Delete {
        #[arg(short, long)]
        /// A bookmark/container
        entity: String,
    },
    /// List the bookmarks
    List,
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
#[clap(about = "Hello from mookbark.run!
Run 'mookbark' without any commands to launch Mookbark terminal app and press '?' to see all the keymaps.")]
#[clap(name = "mookbark")]
pub struct Args {
    #[clap(hide(true))]
    pub server: Option<Server>,
    #[command(subcommand)]
    pub command: Option<Commands>,
}

pub async fn parse_commands<'a>() -> Result<()> {
    let command = Args::parse().command;

    match command {
        Some(command) => match command {
            Commands::Login { oauth_provider } => {
                let (tx, rx) = tokio::sync::oneshot::channel();
                let server = tokio::spawn(crate::cli::auth::create_server::create_server(tx));
                let login = tokio::spawn(crate::cli::auth::login::login(oauth_provider, rx));
                let _ = join(server, login).await;
            }
            Commands::Add { add_command } => match add_command {
                AddSubcommand::Bookmark {
                    container,
                    bookmark,
                } => match cli::add_bookmark::add_bookmark(container.as_deref(), &bookmark).await {
                    Ok(_res) => {}
                    Err(err) => {
                        println!("{err}");
                    }
                },
                AddSubcommand::Container {
                    parent_container,
                    container,
                } => {
                    match cli::add_container::add_container(parent_container.as_deref(), &container)
                        .await
                    {
                        Ok(_res) => {}
                        Err(err) => {
                            println!("{err}");
                        }
                    }
                }
            },
            Commands::Delete { entity } => match cli::delete_entity::delete_entity(&entity).await {
                Ok(_res) => {}
                Err(err) => {
                    println!("{err}");
                }
            },
            Commands::List => match cli::get_bookmarks::get_bookmarks().await {
                Ok(res) => {
                    println!("{res}");
                }
                Err(err) => {
                    println!("{err}");
                }
            },
        },
        None => match terminal_app::run::run().await {
            Ok(_res) => {}
            Err(err) => {
                println!("{err}");
            }
        },
    };
    Ok(())
}
