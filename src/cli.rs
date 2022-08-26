use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Add a project to the ecosystem file
    Add {
        #[clap(flatten)]
        project: crate::ecosystem::Project,
    },
    /// Build the site.
    Build {
        /// The directory in which to build the site.
        #[clap(default_value = "./build")]
        target: String,
        /// Generate a CNAME file with the provided domain
        #[clap(long)]
        cname: Option<String>,
    },
    /// Clean the build directory.
    Clean {
        /// The directory in which the built files are.
        #[clap(default_value = "./build")]
        target: String,
    },
}

pub fn parse() -> Cli {
    Cli::parse()
}
