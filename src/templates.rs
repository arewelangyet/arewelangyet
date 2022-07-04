use crate::ecosystem::Topic;
use std::collections::HashMap;
use tera::{from_value, to_value, Error, Function, Tera, Value};

pub fn load(glob: &str, topics: &[Topic]) -> Result<Tera, Error> {
    let mut tera = Tera::new(glob)?;
    tera.register_function("url_for", UrlFor(topics.to_vec()));

    Ok(tera)
}

struct UrlFor(Vec<Topic>);

impl UrlFor {
    fn resolve_url(&self, section: &str, spec: &str) -> Result<String, Error> {
        match section {
            "topic" => {
                match self
                    .0
                    .iter()
                    .filter(|t| t.id == spec)
                    .collect::<Vec<&Topic>>()
                    .first()
                {
                    Some(topic) => Ok(format!("topics/{}", topic.id)),
                    None => Err(format!("Unable to find topic {}", spec).into()),
                }
            }
            _ => Err("Invalid argument for section".into()),
        }
    }
}

impl Function for UrlFor {
    fn call(&self, args: &HashMap<String, Value>) -> tera::Result<Value> {
        let section = match args.get("section") {
            Some(val) => from_value::<String>(val.clone())?,
            None => return Err("required argument `name` not provided".into()),
        };
        let name = match args.get("name") {
            Some(val) => from_value::<String>(val.clone())?,
            None => return Err("required argument `name` not provided".into()),
        };

        Ok(to_value(&self.resolve_url(&section, &name)?)?)
    }

    fn is_safe(&self) -> bool {
        true
    }
}
