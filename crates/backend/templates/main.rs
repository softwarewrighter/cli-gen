use clap::Parser;
use {{ crate_name }}::cli::Cli;

fn main() {
    // Check for version flag with detailed output (handled by sw-cli)
    if sw_cli::check_version_flag() {
        println!("{}", sw_cli::version!());
        return;
    }

    let cli = Cli::parse();

    // TODO: Implement your CLI logic here
    println!("{{ name }} is running!");
    println!("Config: {:?}", cli);
}
