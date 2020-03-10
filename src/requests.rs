use json::JsonValue;
use reqwest::{self, Error, Response};
use reqwest::header::USER_AGENT;

use crate::app::{Config, parse_env};
use crate::PullRequest;

pub async fn get_pull_requests(repo: &str) -> Result<Option<Vec<PullRequest>>, Box<dyn std::error::Error>> {
    let Config { org, pass, user, .. } = parse_env();
    let url = format!(
        "https://{user}:{pass}@api.github.com/repos/{org}/{repo}/pulls?state=open",
        user = user,
        pass = pass,
        org = org,
        repo = repo
    );
    let client = reqwest::Client::new();
    let resp = client.get(&url)
        .header(USER_AGENT, "My Rust Program 1.0")
        .send()
        .await?
        .text()
        .await?;

    if let JsonValue::Array(vec) = json::parse(&resp).unwrap() {
        return Ok(Some(vec.iter().map(|item| {
            let url = item["html_url"].as_str().unwrap().to_string();
            let branch = item["head"]["ref"].as_str().unwrap().to_string();
            let author = item["user"]["login"].as_str().unwrap().to_string();
            PullRequest { url, branch, author }
        }).collect()));
    } else {
        return Ok(None)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use tokio::runtime::Runtime;


    #[test]
    fn call_api() {
        let mut rt = Runtime::new().unwrap();
        if let Ok(Some(vec)) = rt.block_on(get_pull_requests("client-cc-dashboard")) {
            assert_eq!(vec.len(),2);
        } else {
            assert!(false);
        }
    }
}
