use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::error::Error;
use std::fs;

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

#[derive(Serialize, Deserialize, Clone)]
pub struct Project {
    pub name: String,
    pub description: Option<String>,
    pub repo: Option<String>,
    pub crates: Option<Vec<String>>,
    // in case there are separate docs, apart from the crates
    pub docs: Option<String>,
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
    let parsed_data: EcosystemSrc = toml::from_slice(&fs::read(source)?)?;
    let mut topics: Vec<Topic> = parsed_data
        .topics
        .iter()
        .map(|(k, v)| Topic {
            id: k.clone(),
            name: v.name.clone(),
            description: v.description.clone(),
        })
        .collect();
    topics.sort_by(|a, b|
        a.id.to_lowercase().cmp(&b.id.to_lowercase()));

    Ok(Ecosystem {
        projects: parsed_data.projects,
        topics,
        showcase: parsed_data.showcase,
    })
}
