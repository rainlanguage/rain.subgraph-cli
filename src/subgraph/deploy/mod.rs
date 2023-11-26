use crate::subgraph::command;

use crate::subgraph::build::{build, BuildArgs};
use clap::Parser;

/// Arguments for building the yaml file to generate the code used by subgraph
#[derive(Debug, Parser)]
pub struct DeployArgs {
    /// Network that the subgraph will index (default: localhost)
    #[arg(long, required = true)]
    pub network: Option<String>,
    /// Block number where the subgraph will start indexing (default: 0)
    #[arg(long = "block", required = true)]
    pub block_number: Option<u64>,
    /// Contract address that the subgraph will be indexing (default: 0x000..000)
    #[arg(long, required = true)]
    pub address: Option<String>,
    /// Input subgraph template yaml that will be used to build (default: ./subgraph.template.yaml)
    #[arg(long)]
    pub template_path: Option<String>,
    /// Output subgraph yaml that will be used to build (default: ./subgraph.yaml)
    #[arg(long = "output")]
    pub output_path: Option<String>,
    /// The subgraph name that will be use to deploy
    #[arg(long, required = true)]
    pub subgraph_name: String,
    /// The subgraph endpoint URL that will be use to deploy
    #[arg(long)]
    pub endpoint: Option<String>,
    /// The subgraph token access that will be use to deploy
    #[arg(long)]
    pub token_access: Option<String>,
}

pub fn deploy(args: DeployArgs) -> anyhow::Result<()> {
    let build_args = BuildArgs {
        network: args.network,
        block_number: args.block_number,
        address: args.address,
        template_path: args.template_path,
        output_path: args.output_path,
    };

    let resp_build = build(build_args);
    if resp_build.is_err() {
        tracing::error!("{}", resp_build.err().unwrap().to_string());
        std::process::exit(1);
    }

    // Check if token_access is present, if true - grant access
    if args.token_access.is_some() {
        let resp_auth = command::run(
            "npx",
            &[&format!(
                "graph auth --product hosted-service {}",
                args.token_access.unwrap()
            )],
        );
        if resp_auth.is_err() {
            tracing::error!("{}", resp_auth.err().unwrap().to_string());
            std::process::exit(1);
        }
    }

    match args.endpoint {
        Some(endpoint) => {
            if endpoint.contains("localhost") {
                // Create node
                let resp_create = command::run(
                    "bash",
                    &[
                        "-c",
                        &format!(
                            "npx graph create --node {} {}",
                            endpoint, &args.subgraph_name
                        ),
                    ],
                );
                if resp_create.is_err() {
                    tracing::error!("{}", resp_create.err().unwrap().to_string());
                    std::process::exit(1);
                }

                // Deploy with url
                return deploy_sg_with_endpoint(
                    &args.subgraph_name,
                    endpoint,
                    Some("http://localhost:5001".to_string()),
                );
            } else {
                // Deploy just with url
                return deploy_sg_with_endpoint(&args.subgraph_name, endpoint, None);
            }
        }
        None => {
            // Deploy directly (token access previusly checked)
            let resp_deploy = command::run(
                "bash",
                &[
                    "-c",
                    &format!("npx graph deploy --node {}", &args.subgraph_name),
                ],
            );
            if resp_deploy.is_err() {
                tracing::error!("{}", resp_deploy.err().unwrap().to_string());
                std::process::exit(1);
            }

            return Ok(());
        }
    }
}

fn deploy_sg_with_endpoint(
    name: &str,
    subgraph_endpoint: String,
    ipfs_endpoint: Option<String>,
) -> anyhow::Result<()> {
    // Use the IPFS value provided if exist, otherwise just will be an empty value not considered in the command
    let ipfs_url = match ipfs_endpoint {
        Some(endpoint) => format!("--ipfs {}", endpoint),
        None => "".to_string(),
    };

    let resp_deploy = command::run(
        "bash",
        &[
            "-c",
            &format!(
                "npx graph deploy --node {} {} {} --version-label 1",
                subgraph_endpoint, ipfs_url, name
            ),
        ],
    );

    if resp_deploy.is_err() {
        tracing::error!("{}", resp_deploy.err().unwrap().to_string());
        std::process::exit(1);
    }

    Ok(())
}
