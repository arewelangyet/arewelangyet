use clap::Args;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error::Error, fs};
use toml_edit::{Array, Document, Formatted, Item, Table, Value};

#[derive(Deserialize)]
struct EcosystemSrc {
    topics: HashMap<String, TopicSrc>,
    #[serde(rename = "project")]
    projects: Vec<Project>,
    showcase: Vec<ShowcaseExhibit>,
}

pub struct Ecosystem {
    pub topics: Vec<Topic>,
    pub projects: Vec<Project>,
    pub showcase: Vec<ShowcaseExhibit>,
}

#[derive(Args, Clone, Serialize, Deserialize)]
pub struct Project {
    /// The name of the project
    pub name: String,
    /// The description of the project
    #[clap(long, short = 'D')]
    pub description: Option<String>,
    /// A link to the project's repository
    #[clap(long, short = 'R')]
    pub repo: Option<String>,
    /// Crates which are part of this project
    #[clap(name = "CRATE", long = "crate", short = 'c')]
    pub crates: Option<Vec<String>>,
    // in case there are separate docs, apart from the crates
    /// Supply a link to docs not hosted on docs.rs
    #[clap(long, short = 'd')]
    pub docs: Option<String>,
    /// Topics which this crate falls under (e.g. lexing, parsing)
    #[clap(name = "TOPIC", long = "topic", short = 't')]
    pub topics: Vec<String>,
}

#[derive(Deserialize)]
struct TopicSrc {
    name: String,
    description: String,
}

#[derive(Serialize, Clone)]
pub struct Topic {
    pub id: String,
    pub name: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ShowcaseExhibit {
    pub name: String,
    pub repo: Option<String>,
    pub description: Option<String>,
    pub crates: Option<Vec<String>>,
    pub docs: Option<String>,
}

pub fn parse(source: &str) -> Result<Ecosystem, Box<dyn Error>> {
    let parsed_data: EcosystemSrc = toml_edit::easy::from_slice(&fs::read(source)?)?;
    let mut topics: Vec<Topic> = parsed_data
        .topics
        .iter()
        .map(|(k, v)| Topic {
            id: k.clone(),
            name: v.name.clone(),
            description: v.description.clone(),
        })
        .collect();
    topics.sort_by(|a, b| a.id.to_lowercase().cmp(&b.id.to_lowercase()));

    Ok(Ecosystem {
        projects: parsed_data.projects,
        topics,
        showcase: parsed_data.showcase,
    })
}

pub fn add_project(source: &str, project: &Project) -> Result<String, Box<dyn Error>> {
    let mut doc = fs::read_to_string(source)?.parse::<Document>()?;

    macro_rules! value_str {
        ($value:expr) => {
            Item::Value(Value::String(Formatted::new($value.to_string())))
        };
    }
    macro_rules! value_arr {
        ($value:expr) => {
            Item::Value(Value::Array(Array::from_iter($value)))
        };
    }

    let mut table = Table::new();
    table.insert("name", value_str!(&project.name));
    if let Some(description) = &project.description {
        table.insert("description", value_str!(description));
    }
    if let Some(repo) = &project.repo {
        table.insert("repo", value_str!(repo));
    }
    if let Some(crates) = &project.crates {
        table.insert("crates", value_arr!(crates));
    }
    if let Some(docs) = &project.docs {
        table.insert("docs", value_str!(docs));
    }
    table.insert("topics", value_arr!(&project.topics));

    doc["project"].as_array_of_tables_mut().unwrap().push(table);

    Ok(doc.to_string())
}
