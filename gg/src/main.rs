use gg::{get_cmd, get_matches, run};


#[tokio::main]
async fn main() {
    let mut cmd = get_cmd();
    let matches = get_matches(&mut cmd).await;

    run(cmd, matches).await;
}
