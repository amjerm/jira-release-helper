use config::Config;
use dirs;
use jira_release_helper::{process_repository, Arguments, Repository};
use std::collections::HashMap;
use std::env;

fn main() {
    // parse arguments
    let args: Vec<String> = env::args().skip(1).collect();
    let arguments = Arguments::new(&args);

    // run all repositories if no --list flag is used
    let all_repos = arguments.repo_list.is_empty();

    // establish buffer to read config file
    let mut config_buffer = dirs::config_dir().unwrap();

    println!("OS is {}", env::consts::OS);

    match env::consts::OS {
        "windows" => config_buffer.push("jira-release-helper\\Config.toml"),
        _ => config_buffer.push("jira-release-helper/Config.toml"),
    };

    // read settings from config file
    let settings = Config::builder()
        .add_source(config::File::from(config_buffer))
        .build()
        .expect("Unable to read config file");

    // read repositories array from settings
    let repositories = settings
        .get::<Vec<HashMap<String, String>>>("repositories")
        .expect("Error parsing config file");

    let mut all_tickets: Vec<String> = vec![];

    for repository in repositories {
        let label = repository
            .get("label")
            .expect("All repositories must have a label")
            .to_string();
        let mut is_included = false;

        // run repository if found in flags
        if let Some(_) = arguments.repo_list.get(&label) {
            is_included = true;
        };

        if all_repos == true || is_included {
            // set default release branch name
            let mut release_branch = String::from("release");

            // use custom release branch name if set
            if let Some(b) = repository.get("release_branch") {
                release_branch = b.to_string();
            };

            // get tickets for repository
            let mut tickets = process_repository(Repository::new(
                label,
                repository
                    .get("location")
                    .expect("All repositories must have a location")
                    .to_string(),
                repository
                    .get("project_key")
                    .expect("All repositories must have a project_key")
                    .to_string(),
                release_branch,
            ));

            // append tickets to full ticket list
            all_tickets.append(&mut tickets);
        }
    }

    all_tickets.sort();
    all_tickets.dedup();

    println!("\nAll Tickets:");
    println!("{}", all_tickets.join(","));
}
