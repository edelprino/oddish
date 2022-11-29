use serde::{Deserialize};
use octocrab::Octocrab;

#[derive(Deserialize)]
pub struct GithubService {
    token: String,
    username: String,
    repositories: Vec<String>,
}

impl GithubService {
    pub async fn check_all_builds(&self) -> Vec<super::Build> {
        let octocrab = Octocrab::builder().personal_token(self.token.clone()).build().unwrap();
        let mut builds: Vec<super::Build> = Vec::new();
        for repository in &self.repositories {
            let (owner, repo) = repository.split_once('/').unwrap();
            let runs = octocrab.workflows(owner, repo)
                .list_all_runs()
                .actor(self.username.clone())
                .per_page(2)
                .send()
                .await
                .unwrap();
            for run in runs {
                let state = match run.conclusion.unwrap_or("".to_string()).as_str() {
                    "success" => super::BuildState::Success,
                    "failure" => super::BuildState::Failure,
                    "cancelled" => super::BuildState::Failure,
                    _ => super::BuildState::Pending,
                };
                builds.push(super::Build::new(run.id.to_string(), run.head_commit.message, state, run.head_branch, repository.to_string()));
            }
        }
        return builds;
    }
}
