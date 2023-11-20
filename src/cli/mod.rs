use crate::subgraph;
use anyhow::Result;
use clap::command;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Show debug messages
    #[clap(long, short = 'd', global = true)]
    pub debug: bool,
    #[command(subcommand)]
    subgraph: Subgraph,
}

#[derive(Subcommand, Debug)]
pub enum Subgraph {
    /// Build the current subgraph manifest
    Build(subgraph::build::BuildArgs),
}

pub async fn dispatch(subgraph: Subgraph) -> Result<()> {
    match subgraph {
        Subgraph::Build(args) => subgraph::build::build(args),
    }
}

pub async fn main() -> Result<()> {
    // tracing::subscriber::set_global_default(tracing_subscriber::fmt::Subscriber::new())?;

    let cli = Cli::parse();
    if cli.debug {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .with_target(false)
            .init();
    } else {
        tracing::subscriber::set_global_default(tracing_subscriber::fmt::Subscriber::new())?;
    }
    dispatch(cli.subgraph).await
}
