use clap::{Subcommand, Args, Parser};


#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct CliOptions {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Arch(ArchCommand),
}

#[derive(Args)]
#[clap(author, version, about, long_about = None)]
pub struct ArchCommand {
    /// whether build the source
    #[clap(short, long)]
    pub build: Option<bool>,

    /// Install package after successful build
    #[clap(short, long)]
    pub install: bool,

    /// Install missing dependencies with pacman
    #[clap(short, long)]
    pub syncdeps: bool,

    /// Overwrite existing package
    #[clap(short, long)]
    pub force: bool,

    /// Run mksrcinfo
    #[clap(short, long)]
    pub mksrcinfo: bool,

    /// Cargo.toml directory path
    #[clap(short='p', long)]
    pub manifest_path: Option<String>,
}

/// parse CLI arguments
pub fn get_args() -> CliOptions {
    // this will parse CLI args and construct CliOptions
    CliOptions::parse()
}
