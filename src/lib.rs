use regex::Regex;
use std::env;
use std::path::Path;
use std::process::Command;

pub fn process_repository(repository_path: &str) -> Vec<String> {
    // change the working directory
    let repo_path = Path::new(&repository_path);
    assert!(env::set_current_dir(&repo_path).is_ok());

    let git_log: String = get_git_log();
    let tickets: Vec<String> = parse_tickets(git_log);
    tickets
}

fn get_git_log() -> String {
    let mut log_command = Command::new("git");
    log_command
        .arg("log")
        .arg("origin/release")
        .arg("--not")
        .arg("origin/HEAD")
        .arg("--oneline");

    let command_result = log_command.output().expect("git log output failed").stdout;
    let log_result: String = String::from_utf8(command_result).unwrap();
    log_result
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
