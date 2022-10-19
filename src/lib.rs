use regex::Regex;
use std::env;
use std::path::Path;
use std::process::Command;

pub fn process_repository(repository_path: &str) -> Vec<String> {
    // change the working directory
    let repo_path = Path::new(&repository_path);
    assert!(env::set_current_dir(&repo_path).is_ok());
    println!(
        "Successfully changed working directory to {}!",
        repo_path.display()
    );

    // get git log for the repository
    let mut git_log = Command::new("git");
    git_log
        .arg("log")
        .arg("origin/release")
        .arg("--not")
        .arg("origin/HEAD")
        .arg("--oneline");

    // print the result
    let log_result: String =
        String::from_utf8(git_log.output().expect("git log output failed").stdout).unwrap();

    println!("git log result: {}", &log_result);

    let tickets: Vec<String> = parse_tickets(log_result);
    tickets
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
