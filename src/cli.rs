use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Build the site
    Build {
        /// The directory in which to build the site.
        #[clap(default_value = "./build")]
        target: String,
    },
}

pub fn parse() -> Cli {
    Cli::parse()
}
