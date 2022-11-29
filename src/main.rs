mod oddish;

#[tokio::main]
async fn main() {
    let mut repository = oddish::BuildRepository::new();
    loop {
        let config = oddish::Configuration::from_file().unwrap();
        config.check_all_builds(&mut repository).await;
        config.wait().await;
    }
}
