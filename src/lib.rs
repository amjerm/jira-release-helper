use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::path::Path;
use std::process::Command;

#[derive(Clone)]
pub struct Repository {
    pub label: String,
    pub location: String,
    pub project_key: String,
    pub release_branch: String,
}

impl Repository {
    pub fn new(
        label: String,
        location: String,
        project_key: String,
        release_branch: String,
    ) -> Self {
        Self {
            label,
            location,
            project_key,
            release_branch,
        }
    }
}

pub struct Arguments {
    pub repo_list: HashMap<String, bool>,
}

impl Arguments {
    pub fn new(args: &[String]) -> Self {
        let mut repo_list: HashMap<String, bool> = HashMap::new();

        let arguments = args.clone();
        for (i, argument) in args.iter().enumerate() {
            match argument.as_str() {
                "-l" | "--list" => match arguments.get(i + 1) {
                    Some(value) => value.split(',').for_each(|i| {
                        repo_list.insert(i.to_string(), true);
                    }),
                    None => {
                        println!("No value provided for argument {}", argument);
                        std::process::exit(1);
                    }
                },
                _ => (),
            }
        }

        Self { repo_list }
    }
}

pub fn process_repository(repo: Repository) -> Vec<String> {
    let repository_path = repo.location;
    // change the working directory
    let repo_path = Path::new(&repository_path);
    assert!(env::set_current_dir(&repo_path).is_ok());

    let git_log: String = get_git_log(repo.release_branch);
    let mut tickets: Vec<String> = parse_tickets(repo.project_key, git_log);

    tickets.sort();
    tickets.dedup();

    println!("\n{} Tickets:", repo.label);
    println!("{}", tickets.join(","));
    tickets
}

fn get_git_log(release_branch: String) -> String {
    let mut log_command = Command::new("git");
    log_command
        .arg("log")
        .arg(format!("origin/{}", &release_branch))
        .arg("--not")
        .arg("origin/HEAD")
        .arg("--oneline");

    let command_result = log_command.output().expect("git log output failed").stdout;
    String::from_utf8(command_result).unwrap()
}

fn parse_tickets(project_key: String, string: String) -> Vec<String> {
    let formatted_re_str = format!(r"{}-\d*", project_key);
    let re = Regex::new(formatted_re_str.as_str()).unwrap();

    let ticket_matches = re.find_iter(&string);
    let mut tickets: Vec<String> = ticket_matches.map(|x| String::from(x.as_str())).collect();

    tickets.dedup();
    tickets
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_tickets() {
        let mock_git_log =
            String::from("feat(something): do a thing (TI-123)\nfix: repair another (TI-124)");
        let expected: Vec<String> = [String::from("TI-123"), String::from("TI-124")].to_vec();
        assert_eq!(parse_tickets("TI".to_string(), mock_git_log), expected);
    }
}
