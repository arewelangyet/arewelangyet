use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::error::Error;
use std::fs;

pub struct Ecosystem {
    pub topics: Vec<Topic>,
    pub projects: Vec<Project>,
    pub showcase: Vec<Exhibit>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Project {
    pub name: String,
    pub repo: Option<String>,
    pub crates_io: Option<String>,
    pub description: Option<String>,
    pub docs: Option<String>,
    pub topics: Vec<String>,
}

#[derive(Serialize, Clone)]
pub struct Topic {
    pub id: String,
    pub name: String,
    pub description: String,
}

impl PartialEq<Self> for Topic {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Topic {}

impl PartialOrd<Self> for Topic {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.id.partial_cmp(&other.id)
    }
}

impl Ord for Topic {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Exhibit {
    pub name: String,
    pub repo: Option<String>,
    pub description: Option<String>,
    pub crates_io: Option<String>,
    pub docs: Option<String>,
}

#[derive(Deserialize)]
struct EcosystemSrc {
    topics: HashMap<String, TopicSrc>,
    #[serde(rename = "project")]
    projects: Vec<Project>,
    showcase: Vec<Exhibit>,
}

#[derive(Deserialize)]
struct TopicSrc {
    name: String,
    description: String,
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
    topics.sort();

    Ok(Ecosystem {
        projects: parsed_data.projects,
        topics,
        showcase: parsed_data.showcase,
    })
}
