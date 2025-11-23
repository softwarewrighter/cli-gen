use chrono::Datelike;
use clap::Parser;
use cli_codegen_backend::{
    codegen::generators::CodeGenerator,
    models::config::{CliConfig, LicenseType},
};

// CLI Command definitions
#[derive(Parser)]
#[clap(
    version = "1.0",
    about = "CLI Code Generator - Generate standardized CLI applications"
)]
enum CliCommands {
    /// Generate a new CLI project
    #[clap(name = "generate")]
    Generate {
        /// Name of the CLI application
        #[clap(short = 'n', long = "name", default_value = "my-cli")]
        name: String,

        /// Short description of the CLI
        #[clap(short = 's', long = "short-desc")]
        short_description: Option<String>,

        /// Long description of the CLI
        #[clap(long = "long-desc")]
        long_description: Option<String>,

        /// Copyright information
        #[clap(long = "copyright")]
        copyright: Option<String>,

        /// License type (MIT, Apache2, GPL3)
        #[clap(long = "license", default_value = "MIT")]
        license: String,

        /// Output directory for generated code
        #[clap(short = 'o', long = "output", default_value = "./generated_cli")]
        output_dir: String,

        /// Include version support
        #[clap(long = "version-support", action)]
        version_support: bool,

        /// Include help support
        #[clap(long = "help-support", action)]
        help_support: bool,
    },

    /// Serve the web UI
    #[clap(name = "serve")]
    Serve {
        /// Port to run the server on
        #[clap(short = 'p', long = "port", default_value = "3000")]
        port: u16,
    },

    /// Process configurations from a batch file
    #[clap(name = "batch")]
    Batch {
        /// Path to the configuration file
        #[clap(short = 'f', long = "file")]
        config_file: String,

        /// Output directory for generated code
        #[clap(short = 'o', long = "output", default_value = "./generated_cli")]
        output_dir: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli_args = CliCommands::parse();

    match cli_args {
        CliCommands::Generate {
            name,
            short_description,
            long_description,
            copyright,
            license,
            output_dir,
            version_support,
            help_support,
        } => {
            // Create configuration from command line arguments
            let config = CliConfig {
                name,
                short_description: short_description
                    .unwrap_or_else(|| "A brief description of the CLI".to_string()),
                long_description: long_description
                    .unwrap_or_else(|| "A longer description of what this CLI does".to_string()),
                copyright: copyright.unwrap_or_else(|| {
                    format!("Copyright (c) {}", chrono::Utc::now().date_naive().year())
                }),
                license: match license.as_str() {
                    "MIT" => LicenseType::MIT,
                    "Apache2" => LicenseType::Apache2,
                    "GPL3" => LicenseType::GPL3,
                    _ => LicenseType::Custom(license),
                },
                version_support,
                help_support,
            };

            // Generate the CLI code
            CodeGenerator::generate_files(&config, &output_dir)?;
            println!("CLI code generated successfully to: {}", output_dir);

            Ok(())
        }
        CliCommands::Serve { port: _port } => {
            // This command should be handled by the main server binary
            eprintln!("Serve command should be run via the main server binary");
            eprintln!("Run: cargo run -p cli-codegen-backend --bin cli-codegen-backend");
            Ok(())
        }
        CliCommands::Batch {
            config_file,
            output_dir,
        } => {
            // Read configuration from file
            let config_content = std::fs::read_to_string(config_file)?;
            let config: CliConfig = serde_json::from_str(&config_content)?;

            // Validate the configuration
            CodeGenerator::validate_config(&config)?;

            // Generate the CLI code
            CodeGenerator::generate_files(&config, &output_dir)?;
            println!(
                "CLI code generated successfully from batch config to: {}",
                output_dir
            );

            Ok(())
        }
    }
}
