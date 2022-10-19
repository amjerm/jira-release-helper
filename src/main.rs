use jira_release_helper::process_repository;

fn main() {
    let laravel_path = String::from("/Users/adamjermstad/code/tsi/docker/repos/laravel");

    let tickets: Vec<String> = process_repository(&laravel_path);

    println!("laravel tickets");
    println!("{}", tickets.join(","));
}
