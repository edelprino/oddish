use serde::{Deserialize};
use std::fs;
use octocrab::Octocrab;

#[derive(Deserialize)]
struct Configuration {
    services: Services,
}

#[derive(Deserialize)]
struct Services {
    github: Option<GithubService>,
}

#[derive(Deserialize)]
struct GithubService {
    token: String,
    username: String,
    repositories: Vec<String>,
}

impl Configuration {
    fn from_file() -> Result<Self, std::io::Error> {
        fs::read_to_string("./.oddish.toml")
            .and_then(|s| toml::from_str(&s).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e)))
    }

    async fn check_all_pull_requests(self) {

        if let Some(github) = &self.services.github {
            println!("Checking github pull requests");

            let octocrab = Octocrab::builder().personal_token(github.token.clone()).build().unwrap();

            for repository in &github.repositories {

                let (owner, repo) = repository.split_once('/').unwrap();

                let prs = octocrab.pulls(owner, repo)
                    .list()
                    .state(octocrab::params::State::Open)
                    .send()
                    .await
                    .unwrap();

                let runs = octocrab.workflows(owner, repo)
                    .list_all_runs()
                    .actor(github.username.clone())
                    .per_page(2)
                    .send()
                    .await
                    .unwrap();

                for run in runs {
                    println!("{:?}", run.id);
                    println!("{:?}", run.status);
                    println!("{:?}", run.conclusion);
                    println!("{:?}", run.head_branch);
                    println!("{:?}", run);
                    println!("---------------");
                }

                for pr in prs {
                    let author = pr.user.unwrap().login.to_string();
                    if author == github.username {
                        println!("{}", pr.title.unwrap_or_default());
                        // print statuses url
                        println!("{}", pr.statuses_url.unwrap().to_string());
                        println!("{}", pr.head_branch);
                        println!("---------------");
                    }
                }
            }


        }
    }
}

#[tokio::main]
async fn main() {
    let config = Configuration::from_file().unwrap();
    config.check_all_pull_requests().await;
}
