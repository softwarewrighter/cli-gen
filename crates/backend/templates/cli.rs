use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "{{ name }}")]
#[command(author = "{{ author }}")]
#[command(about = sw_cli::short_help!())]
#[command(long_about = sw_cli::long_help!())]
pub struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    pub verbose: bool,

    /// Dry-run mode (show what would be done without doing it)
    #[arg(short = 'n', long)]
    pub dry_run: bool,

    // TODO: Add your CLI-specific arguments here
    // Example:
    // /// Path to input file
    // #[arg(short, long, value_name = "FILE")]
    // pub input: Option<PathBuf>,
}
