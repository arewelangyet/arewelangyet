use std::{
    fs, io,
    path::{Path, PathBuf},
    process::Command,
};
use tera::{self, Context};

mod cli;
mod ecosystem;
mod templates;

const TEMPLATE_SOURCE_GLOB: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*.tera.html");
const ASSET_SOURCE_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/assets");
const ECOSYSTEM_SOURCE_FILE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/ecosystem.toml");
const STYLE_ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/scss/theme.scss");

fn main() {
    let opts = cli::parse();

    match &opts.command {
        cli::Commands::Build { target, cname } => build_site(&PathBuf::from(target), cname),
        cli::Commands::Clean { target } => clean_site(&PathBuf::from(target)),
    }
}

fn build_site(target: &Path, cname: &Option<String>) {
    // First of all, load up the ecosystem file.
    let ecosystem =
        ecosystem::parse(ECOSYSTEM_SOURCE_FILE).expect("Failed to parse ecosystem file.");

    // Load in the templates.
    let tera =
        templates::load(TEMPLATE_SOURCE_GLOB, &ecosystem.topics).expect("Failed to load templates");

    // Build the stylesheets.
    let css_path = target.join("theme.css");

    #[cfg(windows)]
    const CMD: &str = "npm.cmd";
    #[cfg(not(windows))]
    const CMD: &str = "npm";

    let output = Command::new(CMD)
        .arg("exec")
        .arg("sass")
        .arg(STYLE_ROOT)
        .arg(css_path)
        .output()
        .expect("Failed to invoke `npm`. Make sure it's installed and available on your PATH");
    if !output.status.success() {
        eprintln!(
            "`npm exec sass [args] failed with exit code {}. stdout and stderr are below.",
            output.status
        );
        eprintln!("{}", String::from_utf8(output.stdout).unwrap());
        eprintln!("{}", String::from_utf8(output.stderr).unwrap());
        return;
    }

    // Render the templates
    let mut index_config = Context::new();
    index_config.insert("topics", &ecosystem.topics);

    let index = tera
        .render("index.tera.html", &index_config)
        .expect("Failed to render index.tera.html");

    let topics_home = tera
        .render("topics.tera.html", &index_config)
        .expect("Failed to render topics.tera.html");

    let mut topics = vec![];

    for topic in &ecosystem.topics {
        let projects: Vec<&ecosystem::Project> = ecosystem
            .projects
            .iter()
            .filter(|c| c.topics.contains(&topic.id))
            .collect();

        let mut config = Context::new();
        config.insert("topic", &topic);
        config.insert("projects", &projects);

        topics.push((
            tera.render("topic.tera.html", &config)
                .expect("Failed to render template"),
            &topic.id,
        ));
    }

    // Now that templates are rendered, write them to the target directory
    fs::create_dir_all(target).expect("Unable to create target directory");
    fs::write(target.join("index.html"), index).expect("Failed to create index page.");

    let topic_dir = target.join("topics");
    fs::create_dir_all(&topic_dir).expect("failed to create the topic directory");
    fs::write(topic_dir.join("index.html"), topics_home).expect("Failed to create topic home page.");

    for (html, topic_name) in topics {
        let dir = topic_dir.join(topic_name);
        fs::create_dir_all(&dir).expect("Unable to create topic directory.");
        fs::write(dir.join("index.html"), html).expect("Failed to write topic page.");
    }

    fn copy_dir_all(src: &Path, dst: &Path) -> io::Result<()> {
        fs::create_dir_all(&dst)?;
        for entry in fs::read_dir(src)? {
            let entry = entry?;
            if entry.file_type()?.is_dir() {
                copy_dir_all(&entry.path(), &dst.join(entry.file_name()))?;
            } else {
                fs::copy(entry.path(), dst.join(entry.file_name()))?;
            }
        }
        Ok(())
    }

    // Copy in the assets to the target directory as well
    copy_dir_all(ASSET_SOURCE_DIR.as_ref(), &target.join("assets")).expect("Failed to copy assets");

    if let Some(domain) = cname {
        fs::write(target.join("CNAME"), domain).expect("Failed to create CNAME file");
    }
}

fn clean_site(target: &Path) {
    // do nothing if the directory doesn't exist
    fs::remove_dir_all(target).unwrap_or(())
}
