// Copyright 2021-2022 AUDITOR developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use auditor::telemetry::deserialize_log_level;
use serde_aux::field_attributes::deserialize_number_from_string;
use tracing_subscriber::filter::LevelFilter;

#[derive(serde::Deserialize, Debug, Clone)]
pub struct Settings {
    #[serde(default = "default_addr")]
    pub addr: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    #[serde(default = "default_port")]
    pub port: u16,
    #[serde(default = "default_record_prefix")]
    pub record_prefix: String,
    #[serde(default = "default_string")]
    pub site_id: String,
    #[serde(default = "default_components")]
    pub components: Vec<ComponentConfig>,
    #[serde(default = "default_log_level")]
    #[serde(deserialize_with = "deserialize_log_level")]
    pub log_level: LevelFilter,
    pub tls_config: TLSConfig,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct TLSConfig {
    pub use_tls: bool,
    pub ca_cert_path: Option<String>,
    pub client_cert_path: Option<String>,
    pub client_key_path: Option<String>,
}

impl TLSConfig {
    /// Checks if TLS is enabled and required paths are provided.
    pub fn validate_tls_paths(&self) -> Result<(), &'static str> {
        if self.use_tls {
            if self.ca_cert_path.is_none() {
                return Err("ca_cert_path is required when use_tls is true");
            }
            if self.client_cert_path.is_none() {
                return Err("client_cert_path is required when use_tls is true");
            }
            if self.client_key_path.is_none() {
                return Err("client_key_path is required when use_tls is true");
            }
        }
        Ok(())
    }
}

fn default_log_level() -> LevelFilter {
    LevelFilter::INFO
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct ComponentConfig {
    pub name: String,
    pub key: String,
    #[serde(default = "default_score")]
    pub scores: Vec<ScoreConfig>,
    pub only_if: Option<OnlyIf>,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct ScoreConfig {
    pub name: String,
    pub value: f64,
    pub only_if: Option<OnlyIf>,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct OnlyIf {
    pub key: String,
    pub matches: String,
}

fn default_addr() -> String {
    "127.0.0.1".to_string()
}

fn default_port() -> u16 {
    8000
}

fn default_record_prefix() -> String {
    "slurm".to_string()
}

fn default_string() -> String {
    "none".to_string()
}

fn default_score() -> Vec<ScoreConfig> {
    vec![]
}

fn default_components() -> Vec<ComponentConfig> {
    vec![ComponentConfig {
        name: "Cores".into(),
        key: "NumCPUs".into(),
        scores: vec![],
        only_if: None,
    }]
}

/// Loads the configuration from a file `configuration.{yaml,json,toml,...}`
#[tracing::instrument(name = "Loading configuration")]
pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let configuration_directory = base_path
        .join("configuration")
        .join("slurm-epilog-collector");

    let settings = config::Config::builder()
        .add_source(config::File::from(configuration_directory.join("base")).required(false));
    let settings = match std::env::args().nth(1) {
        Some(file) => settings.add_source(
            config::File::from(file.as_ref())
                .required(false)
                .format(config::FileFormat::Yaml),
        ),
        None => settings,
    };
    let settings = settings.add_source(
        config::Environment::with_prefix("AUDITOR")
            .separator("__")
            .prefix_separator("_"),
    );

    settings.build()?.try_deserialize()
}
