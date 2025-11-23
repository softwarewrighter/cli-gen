use crate::models::config::{CliConfig, LicenseType};
use serde::{Deserialize, Serialize};
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

pub struct CodeGenerator;

impl CodeGenerator {
    pub fn generate_files(
        config: &CliConfig,
        output_dir: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        eprintln!("[DEBUG] Creating output directory: {}", output_dir);
        fs::create_dir_all(output_dir)?;
        fs::create_dir_all(format!("{}/src", output_dir))?;
        fs::create_dir_all(format!("{}/scripts", output_dir))?;
        fs::create_dir_all(format!("{}/lib", output_dir))?;

        // Try multiple paths to find templates (handles different working directories)
        let template_paths = vec![
            "templates",                // When running from crates/backend
            "crates/backend/templates", // When running from project root
            "./templates",
        ];

        eprintln!("[DEBUG] Looking for templates directory...");
        let mut template_dir = None;
        for path in &template_paths {
            eprintln!("[DEBUG] Checking path: {}", path);
            if Path::new(path).exists() {
                eprintln!("[DEBUG] Found templates at: {}", path);
                template_dir = Some(path);
                break;
            }
        }

        let template_dir = template_dir.ok_or("Could not find templates directory")?;

        // Get license string for Cargo.toml
        let license_str = match &config.license {
            LicenseType::MIT => "MIT",
            LicenseType::Apache2 => "Apache-2.0",
            LicenseType::GPL3 => "GPL-3.0",
            LicenseType::Custom(s) => s.as_str(),
        };

        // Read and process all templates
        Self::generate_from_template(
            template_dir,
            "main.rs",
            &format!("{}/src/main.rs", output_dir),
            config,
            license_str,
        )?;

        Self::generate_from_template(
            template_dir,
            "cli.rs",
            &format!("{}/src/cli.rs", output_dir),
            config,
            license_str,
        )?;

        Self::generate_from_template(
            template_dir,
            "lib.rs",
            &format!("{}/src/lib.rs", output_dir),
            config,
            license_str,
        )?;

        Self::generate_from_template(
            template_dir,
            "new_build.rs",
            &format!("{}/build.rs", output_dir),
            config,
            license_str,
        )?;

        Self::generate_from_template(
            template_dir,
            "Cargo.toml",
            &format!("{}/Cargo.toml", output_dir),
            config,
            license_str,
        )?;

        Self::generate_from_template(
            template_dir,
            "COPYRIGHT",
            &format!("{}/COPYRIGHT", output_dir),
            config,
            license_str,
        )?;

        Self::generate_from_template(
            template_dir,
            ".gitmodules",
            &format!("{}/.gitmodules", output_dir),
            config,
            license_str,
        )?;

        Self::generate_from_template(
            template_dir,
            ".gitignore",
            &format!("{}/.gitignore", output_dir),
            config,
            license_str,
        )?;

        Self::generate_from_template(
            template_dir,
            "short-help.txt",
            &format!("{}/src/short-help.txt", output_dir),
            config,
            license_str,
        )?;

        Self::generate_from_template(
            template_dir,
            "long-help.txt",
            &format!("{}/src/long-help.txt", output_dir),
            config,
            license_str,
        )?;

        // Generate scripts with executable permissions
        Self::generate_from_template(
            template_dir,
            "setup.sh",
            &format!("{}/scripts/setup.sh", output_dir),
            config,
            license_str,
        )?;
        Self::set_executable(&format!("{}/scripts/setup.sh", output_dir))?;

        Self::generate_from_template(
            template_dir,
            "build.sh",
            &format!("{}/scripts/build.sh", output_dir),
            config,
            license_str,
        )?;
        Self::set_executable(&format!("{}/scripts/build.sh", output_dir))?;

        eprintln!("[DEBUG] All files generated successfully");
        Ok(())
    }

    fn generate_from_template(
        template_dir: &str,
        template_name: &str,
        output_path: &str,
        config: &CliConfig,
        license_str: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        eprintln!("[DEBUG] Generating {} from template", output_path);
        let template_path = format!("{}/{}", template_dir, template_name);
        let template_content = fs::read_to_string(&template_path)
            .map_err(|e| format!("Failed to read {}: {}", template_path, e))?;

        // Convert package name to valid crate name (replace - with _)
        let crate_name = config.name.replace("-", "_");

        let content = template_content
            .replace("{{ name }}", &config.name)
            .replace("{{ crate_name }}", &crate_name)
            .replace("{{ short_description }}", &config.short_description)
            .replace("{{ long_description }}", &config.long_description)
            .replace("{{ author }}", &config.author)
            .replace("{{ copyright }}", &config.copyright)
            .replace("{{ license }}", license_str)
            .replace("{{ repository }}", &config.repository)
            .replace("{{ version }}", &config.version)
            .replace("{{ sw_cli_url }}", &config.sw_cli_url);

        fs::write(output_path, content)?;
        Ok(())
    }

    fn set_executable(path: &str) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(unix)]
        {
            let metadata = fs::metadata(path)?;
            let mut permissions = metadata.permissions();
            permissions.set_mode(0o755);
            fs::set_permissions(path, permissions)?;
        }
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
