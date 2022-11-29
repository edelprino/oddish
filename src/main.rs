use serde::{Deserialize};
use std::{fs, collections::HashMap};
mod github;
use std::process::Command;

#[derive(Deserialize)]
struct Configuration {
    services: Services,
    command: String,
}

#[derive(Deserialize)]
struct Services {
    github: Option<github::GithubService>,
}

#[derive(Debug)]
enum BuildState {
    Success,
    Failure,
    Pending,
}

#[derive(Debug)]
pub struct Build {
    id: String,
    commit: String,
    state: BuildState,
    branch: String,
    repository: String,
}

impl Build {
    fn new(id: String, commit: String, state: BuildState, branch: String, repository: String) -> Build {
        Build { id, commit, state, branch, repository}
    }
}

pub type State = HashMap<String, String>;

impl Configuration {

    fn from_file() -> Result<Self, std::io::Error> {
        fs::read_to_string("./.oddish.toml")
            .and_then(|s| toml::from_str(&s).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e)))
    }

    async fn check_all_builds(self, _state: &mut State) {
        let notify = self.command.replace("{message}", "Hello, world!");
        let args = notify.split_whitespace().into_iter().map(|s| s.to_string()).collect::<Vec<String>>();
        let status = Command::new(args[0].clone()).args(&args[1..]).status().unwrap();
        println!("process finished with: {status}");

        if let Some(github) = &self.services.github {
            let builds = github.check_all_builds().await;
            for build in builds {
                println!("{:?}", build);
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let mut state = State::new();
    let config = Configuration::from_file().unwrap();
    config.check_all_builds(&mut state).await;
}
