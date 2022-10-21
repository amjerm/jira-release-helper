use regex::Regex;
use std::env;
use std::path::Path;
use std::process::Command;

#[derive(Clone)]
pub struct Repository {
    pub label: String,
    pub location: String,
    pub release_branch: String,
}

impl Repository {
    pub fn new(label: String, location: String, release_branch: String) -> Self {
        Self {
            label,
            location,
            release_branch,
        }
    }
}

pub fn process_repository(repo: Repository) -> Vec<String> {
    let repository_path = repo.location;
    // change the working directory
    let repo_path = Path::new(&repository_path);
    assert!(env::set_current_dir(&repo_path).is_ok());

    let git_log: String = get_git_log(repo.release_branch);
    let tickets: Vec<String> = parse_tickets(git_log);
    println!("{} Tickets:", repo.label);
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

fn parse_tickets(string: String) -> Vec<String> {
    let re = Regex::new(r"(TI|IG)-\d*").unwrap();
    // replace with tests
    assert!(re.is_match("TI-123"));
    assert!(re.is_match("IG-123"));

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
            String::from("feat(something): do a thing (TI-123)\nfix: repair another (IG-123)");
        let expected: Vec<String> = [String::from("TI-123"), String::from("IG-123")].to_vec();
        assert_eq!(parse_tickets(mock_git_log), expected);
    }
}
