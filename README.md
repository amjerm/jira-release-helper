# jira-release-helper

## IN PROGRESS

**This tool is not complete, and not yet intended for use!!**

A tool to create and manage releases in Jira based on Git commit history.

## Installation

_TBD_

## Configuration

**You _must_ include a config file.**

Add a file named `Config.toml` in a directory named `jira-release-helper` in the traditional config directory for your operating system (as defined by the [dirs](https://docs.rs/dirs/latest/dirs/fn.config_dir.html) Rust crate).

| **Platform**| **Value**                                                             | **Example**                                                               |
|-------------|-----------------------------------------------------------------------|---------------------------------------------------------------------------|
| Linux       | `$XDG_CONFIG_HOME` or `$HOME`/.config/jira-release-helper/Config.toml | /home/alice/.config/jirea-release-helper/Config.toml                      |
| macOS       | `$HOME`/Library/Application Support/jira-release-helper/Config.toml   | /Users/Alice/Library/Application Support/jira-release-helper/Config.toml  |
| Windows     | `{FOLDERID_RoamingAppData}`/jira-release-helper/Config.toml           | C:\Users\Alice\AppData\Roaming/jira-release-helper/Config.toml            |

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
