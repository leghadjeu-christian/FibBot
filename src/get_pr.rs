use reqwest::blocking::Client;
use std::env;
use octocrab::issues;

#[tokio::main]

pub async fn get_pr_body(owner: &str, repo: &str) -> Result<String, Box<dyn std::error::Error>> {

    let client = reqwest::Client::new();

    
     let response = octocrab::instance().pulls(format!("{}", owner).as_str(), format!("{}", repo).as_str()).list_files(1).await;
      // println!("Status Code: {}", response.status());

     let response_body = response.unwrap().items.first().unwrap().patch.clone().unwrap();

      // let response_body =
     //     serde_json::from_str::<Vec<GitHubIssue>>(&response_body).expect("Error serializing to JSON");

     println!("Response body: \n{response_body}");
     Ok(response_body)
}
