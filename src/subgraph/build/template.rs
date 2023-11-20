use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SubgraphTemplate {
    #[serde(rename = "specVersion")]
    spec_version: String,
    schema: Schema,
    #[serde(rename = "dataSources")]
    pub data_sources: Vec<DataSource>,
    pub templates: Vec<DataSource>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Schema {
    file: String,
}

/// Struct definition for DataSource and Template fields in subgraph YAML file
/// that describe every field for generated code
#[derive(Debug, Serialize, Deserialize)]
pub struct DataSource {
    kind: String,
    name: String,
    pub network: String,
    pub source: Source,
    mapping: Mapping,
}

#[derive(Debug, Serialize, Deserialize)]
struct Mapping {
    kind: String,
    #[serde(rename = "apiVersion")]
    api_version: String,
    language: String,
    entities: Vec<String>,
    abis: Vec<Abi>,
    #[serde(rename = "eventHandlers")]
    event_handlers: Vec<EventHandler>,
    file: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Abi {
    name: String,
    file: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct EventHandler {
    event: String,
    handler: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    receipt: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Source {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    abi: String,
    #[serde(rename = "startBlock", skip_serializing_if = "Option::is_none")]
    pub start_block: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Template {
    network: String,
    source: Source,
}
