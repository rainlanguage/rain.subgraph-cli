use serde::{Deserialize, Serialize};
use indoc::indoc;

#[test]
fn yaml_serialization_string_single_quotes() -> anyhow::Result<()> {
    // Initial YAML
    let yaml = "
      specVersion: 0.0.5
      address: '0xff0000000000000000000bb000000000000000cc'
    ";

    // Address to change in the YAML
    let new_address = "0xC3F675E9610e3E1f00874b1dD46BcEa6aFC57049".to_string();

    #[derive(Debug, Serialize, Deserialize)]
    struct Schema {
        #[serde(rename = "specVersion")]
        spec_version: String,
        address: String,
    }

    // Deserialize the YAML text
    let mut yaml_data: Schema = serde_yaml::from_str(yaml)?;

    // Change the address
    yaml_data.address = format!("'{}'", new_address);

    // Serialize back to YAML text
    let yaml_resp = serde_yaml::to_string(&yaml_data)?;

    assert!(
        !yaml_resp.contains(&format!("'''{}'''", new_address)),
        "string with single quotes serialize with multiple single quotes"
    );

    Ok(())
}

#[test]
fn yaml_serialization_string_round_trip() -> anyhow::Result<()> {
    #[derive(Debug, Serialize, Deserialize)]
    struct YamlData {
        #[serde(rename = "specVersion")]
        spec_version: String,
        address: String,
    }

    // Initial YAML
    let yaml = indoc! {r#"
      specVersion: 0.0.5
      address: 0xff0000000000000000000bb000000000000000cc
    "#};

    // Deserialize the YAML text
    let mut yaml_data0: YamlData = serde_yaml::from_str(yaml)?;

    let new_address = "0xC3F675E9610e3E1f00874b1dD46BcEa6aFC57049";

    yaml_data0.address = new_address.into();

    // Serialize back to YAML text
    let yaml_resp = serde_yaml::to_string(&yaml_data0)?;

    let yaml_data1: YamlData = serde_yaml::from_str(&yaml_resp)?;

    assert_eq!(yaml_data0.address, yaml_data1.address);
    assert_eq!(yaml_data1.address, new_address);

    Ok(())
}
