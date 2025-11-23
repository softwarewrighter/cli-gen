use crate::models::config::CliConfig;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

pub struct CodeGenerator;

impl CodeGenerator {
    pub fn generate_files(
        config: &CliConfig,
        output_dir: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        fs::create_dir_all(output_dir)?;

        // Try multiple paths to find templates (handles different working directories)
        let template_paths = vec![
            "templates",                // When running from crates/backend
            "crates/backend/templates", // When running from project root
            "./templates",
        ];

        let mut template_dir = None;
        for path in &template_paths {
            if Path::new(path).exists() {
                template_dir = Some(path);
                break;
            }
        }

        let template_dir = template_dir.ok_or("Could not find templates directory")?;

        // Read template files
        let build_template = std::fs::read_to_string(format!("{}/build.rs", template_dir))?;
        let config_template = std::fs::read_to_string(format!("{}/config.rs", template_dir))?;
        let dispatch_template = std::fs::read_to_string(format!("{}/dispatch.rs", template_dir))?;

        // Perform simple string replacements
        let build_content = build_template
            .replace("{{ version }}", "0.1.0")
            .replace("{{ name }}", &config.name)
            .replace("{{ git_hash }}", "unknown");

        let config_content = config_template
            .replace("{{ long_description }}", &config.long_description)
            .replace("{{ name }}", &config.name)
            .replace("{{ short_description }}", &config.short_description);

        let dispatch_content = dispatch_template;

        // Write generated files to the output directory
        let build_path = Path::new(output_dir).join("build.rs");
        fs::write(build_path, build_content)?;

        let config_path = Path::new(output_dir).join("config.rs");
        fs::write(config_path, config_content)?;

        let dispatch_path = Path::new(output_dir).join("dispatch.rs");
        fs::write(dispatch_path, dispatch_content)?;

        Ok(())
    }

    pub fn validate_config(config: &CliConfig) -> Result<(), String> {
        if config.name.is_empty() {
            return Err("CLI name cannot be empty".to_string());
        }

        if config.short_description.is_empty() {
            return Err("Short description cannot be empty".to_string());
        }

        if config.long_description.is_empty() {
            return Err("Long description cannot be empty".to_string());
        }

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GeneratedFiles {
    pub build_rs: String,
    pub config_rs: String,
    pub dispatch_rs: String,
}
