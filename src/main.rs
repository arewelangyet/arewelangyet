use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
};
use tera::{self, Context};
use toml;

const DEFAULT_OUTPUT_DIR: &str = "./build";
const TEMPLATE_SOURCE_GLOB: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*.tera.html");
const ECOSYSTEM_SOURCE_FILE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/ecosystem.toml");

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Build {
        #[clap(default_value_t = DEFAULT_OUTPUT_DIR.to_string())]
        target: String,
    },
}

#[derive(Serialize, Deserialize)]
struct Ecosystem {
    topics: Vec<Topic>,
    projects: Vec<Project>,
}

#[derive(Serialize, Deserialize)]
struct Project {
    name: String,
    repo: Option<String>,
    crates_io: Option<String>,
    description: Option<String>,
    docs: Option<String>,
    topics: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct Topic {
    name: String,
    description: String,
}

fn main() {
    let opts: Cli = Cli::parse();

    match &opts.command {
        Commands::Build { target } => build_site(&PathBuf::from(target)),
    }
}

#[derive(Serialize)]
struct IndexTemplateArgs<'a> {
    topics: &'a Vec<Topic>,
}

#[derive(Serialize)]
struct TopicTemplateArgs<'a> {
    name: &'a str,
    description: &'a str,
    projects: Vec<&'a Project>,
}

fn build_site(target: &Path) {
    // First of all, load up the ecosystem file.
    let ecosystem: Ecosystem =
        toml::from_slice(&fs::read(ECOSYSTEM_SOURCE_FILE).expect("Failed to read ecosystem.toml"))
            .expect("Failed to parse ecosystem.toml");

    // Load in the templates
    let mut templates =
        tera::Tera::new(TEMPLATE_SOURCE_GLOB).expect("Failed to parse Tera templates");
    templates.autoescape_on(vec![".tera.html"]);

    let index = templates
        .render(
            "index.tera.html",
            &Context::from_serialize(IndexTemplateArgs {
                topics: &ecosystem.topics,
            })
            .unwrap(),
        )
        .expect("Failed to render index.tera.html");

    let mut topics = vec![];

    for topic in &ecosystem.topics {
        let crates: Vec<&Project> = ecosystem
            .projects
            .iter()
            .filter_map(|c| {
                if c.topics.contains(&topic.name) {
                    Some(c)
                } else {
                    None
                }
            })
            .collect();

        let config = TopicTemplateArgs {
            name: &topic.name,
            description: &topic.description,
            projects: crates,
        };

        topics.push((
            templates
                .render("topic.tera.html", &Context::from_serialize(config).unwrap())
                .expect("Failed to render template"),
            &topic.name,
        ));
    }

    // Now that templates are rendered, write them to the target directory
    fs::create_dir_all(target).expect("Unable to create target directory");
    fs::write(target.join("index.html"), index).expect("Failed to create index page.");

    let topic_dir = target.join("topics");

    for (html, topic_name) in topics {
        let dir = topic_dir.join(topic_name);
        fs::create_dir_all(&dir).expect("Unable to create topic directory.");
        fs::write(dir.join("index.html"), html).expect("Failed to write topic page.");
    }
}
