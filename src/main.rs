use config::Config;
use dirs;
use jira_release_helper::{process_repository, Arguments, Repository};
use std::collections::HashMap;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let arguments = Arguments::new(&args);
    let all_repos = arguments.repo_list.is_empty();

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
        let label = repository.get("label").unwrap().to_string();
        let mut is_included = false;

        // run repository if found in flags
        match arguments.repo_list.get(&label) {
            Some(_) => {
                is_included = true;
            }
            None => {}
        };

        if all_repos == true || is_included {
            let mut release_branch = "release".to_string();

            match repository.get("release_branch") {
                Some(b) => release_branch = b.to_string(),
                None => {}
            }

            let mut tickets = process_repository(Repository::new(
                label,
                repository.get("location").unwrap().to_string(),
                repository.get("project_key").unwrap().to_string(),
                release_branch,
            ));

            all_tickets.append(&mut tickets);
        }
    }

    all_tickets.sort();
    all_tickets.dedup();

    println!("\nAll Tickets:");
    println!("{}", all_tickets.join(","));
}
