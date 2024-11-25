use std::{hint::black_box, str::FromStr};

use nix_rs::{command::NixCmd, flake::url::FlakeUrl};
use omnix_common::config::OmConfig;

fn main() {
    divan::main();
}

#[divan::bench]
fn from_json_local_flake() {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let flake_url = FlakeUrl::from_str("../../.").unwrap();

    runtime.block_on(async {
        let om_config = OmConfig::from_json(NixCmd::get().await, &flake_url)
            .await
            .unwrap();
        black_box(om_config);
    });
}

#[divan::bench]
fn from_json_remote_flake() {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let flake_url = FlakeUrl::from_str("github:shivaraj-bh/omnix/omconfig-bench").unwrap();

    runtime.block_on(async {
        let om_config = OmConfig::from_json(NixCmd::get().await, &flake_url)
            .await
            .unwrap();
        black_box(om_config);
    });
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
fn from_yaml_local_flake() {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let flake_url = FlakeUrl::from_str("../../.").unwrap();

    runtime.block_on(async {
        let om_config = OmConfig::from_yaml(NixCmd::get().await, &flake_url)
            .await
            .unwrap();
        black_box(om_config);
    });
}

#[divan::bench]
fn from_yaml_remote_flake() {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let flake_url = FlakeUrl::from_str("github:shivaraj-bh/omnix/omconfig-bench").unwrap();

    runtime.block_on(async {
        let om_config = OmConfig::from_yaml(NixCmd::get().await, &flake_url)
            .await
            .unwrap();
        black_box(om_config);
    });
}
