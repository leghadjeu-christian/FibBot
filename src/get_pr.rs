#[tokio::main]

pub async fn get_pr_body(owner: &str, repo: &str) -> Result<String, Box<dyn std::error::Error>> {
    let response = octocrab::instance().pulls(owner, repo).list_files(1).await;

    let response_body = response
        .unwrap()
        .items
        .first()
        .unwrap()
        .patch
        .clone()
        .unwrap();

    println!("Response body: \n{response_body}");
    Ok(response_body)
}
