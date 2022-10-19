# jira-release-helper

## IN PROGRESS

**This tool is not complete, and not yet intended for use!!**

A tool to create and manage releases in Jira based on Git commit history.

## Installation

_TBD_

## Configuration

**You _must_ include a config file.**

Add a file named `Config.toml` in a directory named `jira-release-helper` in the traditional config directory for your operating system (as defined by the [dirs](https://docs.rs/dirs/latest/dirs/fn.config_dir.html) Rust crate).

| *Platform*  | *Value*                               | *Example*                                 |
|-------------|---------------------------------------|-------------------------------------------|
| Linux       | `$XDG_CONFIG_HOME` or `$HOME`/.config | /home/alice/.config                       |
| macOS       | `$HOME`/Library/Application Support   | /Users/Alice/Library/Application Support  |
| Windows     | `{FOLDERID_RoamingAppData}`           | C:\Users\Alice\AppData\Roaming            |

The file should contain an array of repositories to check written in toml.

Example:

```toml
[[repositories]]
label = "First Repository"
location = "/Users/amjerm/code/first-repo"

[[repositories]]
label = "Second"
location = "/Users/amjerm/code/second"

[[repositories]]
label = "Another"
location = "/Users/amjerm/code/another-repo"
```
