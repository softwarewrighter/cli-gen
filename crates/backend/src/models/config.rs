use chrono::{self, Datelike};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CliConfig {
    pub name: String,
    pub short_description: String,
    pub long_description: String,
    pub copyright: String,
    pub license: LicenseType,
    pub version_support: bool,
    pub help_support: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum LicenseType {
    MIT,
    Apache2,
    GPL3,
    Custom(String),
}

impl Default for CliConfig {
    fn default() -> Self {
        Self {
            name: "my-cli".to_string(),
            short_description: "A brief description of the CLI".to_string(),
            long_description: "A longer description of what this CLI does".to_string(),
            copyright: format!("Copyright (c) {}", chrono::Utc::now().date_naive().year())
                .to_string(),
            license: LicenseType::MIT,
            version_support: true,
            help_support: true,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GeneratedFiles {
    pub build_rs: String,
    pub config_rs: String,
    pub dispatch_rs: String,
}
