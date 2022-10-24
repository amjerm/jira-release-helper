use config::Config;
use dirs;
use jira_release_helper::{process_repository, Repository};
use std::collections::HashMap;
use std::process;

fn main() {
    let config_buffer = dirs::config_dir()
        .unwrap()
        .join("jira-release-helper/Config.toml");

    let settings = match Config::builder()
        .add_source(config::File::from(config_buffer))
        .build()
    {
        Ok(config_obj) => config_obj,
        Err(err) => {
            println!("Error loading config file: {:?}", err);
            process::exit(1);
        }
    };

    let repositories = settings
        .get::<Vec<HashMap<String, String>>>("repositories")
        .unwrap();

    let mut all_tickets: Vec<String> = vec![];

    for repository in repositories {
        let mut release_branch = "release".to_string();

        match repository.get("release_branch") {
            Some(b) => release_branch = b.to_string(),
            None => {}
        }

        let mut tickets = process_repository(Repository::new(
            repository.get("label").unwrap().to_string(),
            repository.get("location").unwrap().to_string(),
            repository.get("project_key").unwrap().to_string(),
            release_branch,
        ));

        all_tickets.append(&mut tickets);
    }

    all_tickets.sort();
    all_tickets.dedup();

    println!("\nAll Tickets:");
    println!("{}", all_tickets.join(","));
}
