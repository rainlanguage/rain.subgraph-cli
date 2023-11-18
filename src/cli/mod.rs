use anyhow::Result;
use clap::command;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    subgraph: Subgraph,
}

#[derive(Subcommand, Debug)]
pub enum Subgraph {
    #[command(about = "Build the current subgraph manifest and code")]
    Build,
}

pub async fn dispatch(subgraph: Subgraph) -> Result<()> {
    match subgraph {
        Subgraph::Build => {
            println!("testing call for build");
            Ok(())
        }
    }
}

pub async fn main() -> Result<()> {
    tracing::subscriber::set_global_default(tracing_subscriber::fmt::Subscriber::new())?;

    let cli = Cli::parse();
    dispatch(cli.subgraph).await
}
