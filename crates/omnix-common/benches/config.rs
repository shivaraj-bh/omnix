use std::{hint::black_box, str::FromStr};

use nix_rs::{command::NixCmd, flake::url::FlakeUrl};
use omnix_common::config::OmConfig;

fn main() {
    divan::main();
}

#[divan::bench]
fn from_json() {
    let json_path = "../../om.json";
    let flake_url = FlakeUrl::from_str("path:../../.").unwrap();

    black_box(OmConfig::from_json(json_path, &flake_url).unwrap());
}

#[divan::bench]
fn from_flake_url() {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let flake_url = FlakeUrl::from_str("../../.").unwrap();

    runtime.block_on(async {
        let om_config = OmConfig::from_flake_url(NixCmd::get().await, &flake_url)
            .await
            .unwrap();
        black_box(om_config);
    });
}

#[divan::bench]
fn from_yaml() {
    let yaml_path = "../../om.yaml";
    let flake_url = FlakeUrl::from_str("../../.").unwrap();

    black_box(OmConfig::from_yaml(yaml_path, &flake_url).unwrap());
}
