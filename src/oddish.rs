use serde::{Deserialize};
use std::{fs, collections::HashMap};
use std::process::Command;
use tokio::time::{sleep, Duration};


#[path = "./github.rs"]
mod github;

#[derive(Deserialize)]
pub struct Configuration {
    services: Services,
    command: String,
    every: u64,
}

#[derive(Deserialize)]
struct Services {
    github: Option<github::GithubService>,
}

#[derive(Debug, PartialEq)]
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

pub struct BuildRepository {
    builds: HashMap<String, Build>,
}

impl BuildRepository {
    pub fn new() -> BuildRepository {
        BuildRepository { builds: HashMap::new() }
    }

    fn add(&mut self, build: Build) {
        self.builds.insert(build.id.clone(), build);
    }

    fn get(&self, id: &str) -> Option<&Build> {
        self.builds.get(id)
    }
}


impl Configuration {

    pub fn from_file() -> Result<Self, std::io::Error> {
        fs::read_to_string("./.oddish.toml")
            .and_then(|s| toml::from_str(&s).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e)))
    }

    pub async fn check_all_builds(&self, repository: &mut BuildRepository) {

        if let Some(github) = &self.services.github {
            let builds = github.check_all_builds().await;
            for build in builds {
                repository.get(&build.id).map(|b| {
                    if b.state != build.state {
                        let message = format!("Build {} changed state from {:?} to {:?}", b.id, b.state, build.state);
                        println!("Running command: {}", self.command);
                        println!("Commit: {}", build.commit);
                        println!("Branch: {}", build.branch);
                        let notify = self.command.replace("{message}", &message);
                        let status = Command::new("bash").arg("-c").arg(notify).status().unwrap();
                        println!("process finished with: {status}");
                    }
                });
                repository.add(build);
            }
        }
    }

    pub async fn wait(self) {
        sleep(Duration::from_secs(self.every)).await;
    }
}
