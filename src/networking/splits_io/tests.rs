use super::{Client, download_run};

#[tokio::test]
async fn can_download_splits() {
    let client = Client::new();
    let run = download_run(&client, "4cg").await.unwrap().run;
    assert_eq!(run.game_name(), "Portal");
    assert_eq!(run.category_name(), "Inbounds");
    assert_eq!(run.attempt_count(), 14);
}