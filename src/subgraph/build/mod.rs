mod template;
use crate::subgraph::command;

use anyhow::anyhow;
use clap::Parser;
use std::fs::File;
use std::io::{ErrorKind, Read, Write};
use template::SubgraphTemplate;

/// Arguments for building the yaml file to generate the code used by subgraph
#[derive(Debug, Parser)]
pub struct BuildArgs {
    /// Network that the subgraph will index (default: localhost)
    #[arg(long)]
    pub network: Option<String>,
    /// Block number where the subgraph will start indexing (default: 0)
    #[arg(long = "block")]
    pub block_number: Option<u64>,
    /// Contract address that the subgraph will be indexing (default: 0x000..000)
    #[arg(long)]
    pub address: Option<String>,
    /// Input subgraph template yaml that will be used to build (default: ./subgraph.template.yaml)
    #[arg(long)]
    pub template_path: Option<String>,
    /// Output subgraph yaml that will be used to build (default: ./subgraph.yaml)
    #[arg(long = "output")]
    pub output_path: Option<String>,
}

/// Build the source for a subgraph code
pub fn build(args: BuildArgs) -> anyhow::Result<()> {
    let resp_gen_sg_yaml = generate_subgraph_yaml(args);
    if resp_gen_sg_yaml.is_err() {
        tracing::error!("{}", resp_gen_sg_yaml.err().unwrap().to_string());
        std::process::exit(1);
    }
    let resp_codegen_cmd = command::run("npm", &["run", "codegen"]);
    if resp_codegen_cmd.is_err() {
        tracing::error!("{}", resp_codegen_cmd.err().unwrap().to_string());
        std::process::exit(1);
    }

    let resp_build_cmd = command::run("npm", &["run", "build"]);
    if resp_build_cmd.is_err() {
        tracing::error!("{}", resp_build_cmd.err().unwrap().to_string());
        std::process::exit(1);
    }

    Ok(())
}

fn generate_subgraph_yaml(args: BuildArgs) -> anyhow::Result<()> {
    let template_path = args
        .template_path
        .unwrap_or("./subgraph.template.yaml".to_string());

    let file_resp = File::open(&template_path);

    let mut file = match file_resp {
        Err(error) => {
            if error.kind() == ErrorKind::NotFound {
                return Err(anyhow!("subgraph template yaml file not found - {}", error));
            }
            return Err(error.into());
        }
        Ok(data) => data,
    };

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Using a default values just to build
    let block_number = args.block_number.unwrap_or(0);
    let network = args.network.unwrap_or("localhost".to_string());

    let address: &str = &args
        .address
        .unwrap_or("0x0000000000000000000000000000000000000000".to_string());

    let mut yaml_data: SubgraphTemplate = serde_yaml::from_str(&contents)?;
    // Update values in dataSources using the given arguments
    for data_source in &mut yaml_data.data_sources {
        data_source.network = network.clone();
        data_source.source.address = Some(address.into());
        data_source.source.start_block = Some(block_number);
    }

    // Update values in templates using the given arguments if the yaml have templates
    if let Some(templates) = &mut yaml_data.templates {
        for template in templates {
            template.network = network.clone();
        }
    }

    let modified_yaml = serde_yaml::to_string(&yaml_data)?;

    let output_path = args.output_path.unwrap_or("./subgraph.yaml".to_string());
    let mut modified_file = File::create(output_path)?;

    modified_file.write_all(modified_yaml.as_bytes())?;
    Ok(())
}
