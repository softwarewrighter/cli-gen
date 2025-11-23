use clap::{Parser, Subcommand};
use std::error::Error;

/// {{ long_description }}
#[derive(Parser)]
#[clap(name = "{{ name }}", version = env!("CLI_VERSION"), about = "{{ short_description }}")]
pub struct CliConfig {
    #[clap(subcommand)]
    pub command: Option<Commands>,
    
    /// Show version information
    #[clap(short = 'V', long = "version", global = true)]
    pub show_version: bool,
    
    /// Verbose mode
    #[clap(short = 'v', long = "verbose", global = true)]
    pub verbose: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Example command
    #[clap(name = "example", about = "An example command")]
    Example {
        /// Example argument
        #[clap(short = 'n', long = "name", default_value = "world")]
        name: String,
    },
}

impl CliConfig {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let config = Self::parse();
        Ok(config)
    }
    
    pub fn execute(&self) -> Result<(), Box<dyn Error>> {
        if self.show_version {
            println!("{{ name }} v{}", env!("CLI_VERSION"));
            return Ok(());
        }
        
        match &self.command {
            Some(Commands::Example { name }) => {
                println!("Hello, {}!", name);
            }
            None => {
                // Default behavior when no subcommand is provided
                println!("{{ short_description }}");
                println!("Use --help for more information.");
            }
        }
        
        Ok(())
    }
}

// Builder pattern implementation
pub struct CliConfigBuilder {
    show_version: bool,
    verbose: bool,
    command: Option<Commands>,
}

impl CliConfigBuilder {
    pub fn new() -> Self {
        Self {
            show_version: false,
            verbose: false,
            command: None,
        }
    }
    
    pub fn show_version(mut self, show: bool) -> Self {
        self.show_version = show;
        self
    }
    
    pub fn verbose(mut self, verbose: bool) -> Self {
        self.verbose = verbose;
        self
    }
    
    pub fn command(mut self, command: Commands) -> Self {
        self.command = Some(command);
        self
    }
    
    pub fn build(self) -> CliConfig {
        CliConfig {
            show_version: self.show_version,
            verbose: self.verbose,
            command: self.command,
        }
    }
}