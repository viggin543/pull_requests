use json::{self, array, JsonValue, object};
use std::fs;
use crate::app::parse_env;
use std::error::Error;
use std::ffi::OsString;

mod app;
mod requests;

#[derive(Debug)]
pub struct PullRequest {
    pub url: String,
    pub branch: String,
    pub author: String,
}


//todo: #[tokio::main] -? how this works ? annotationg functions ?
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let repos = local_repos()?;

    for i in repos {
        println!("{}",i);
    }


    if let Some(vec) = requests::get_pull_requests("svc-operator").await? {
        let items: Vec<JsonValue> = vec.iter().map(|item| {
            let PullRequest { url, branch, author } = item;
            object! {
             "title" => url.as_str(),
             "subtitle" => branch.as_str(),
             "arg" => url.as_str(),
            }
        }).collect();

        println!("{}", object! {
            "items" => array![items]
        });
    }
    Ok(())
}

fn local_repos() -> Result<Vec<String>,Box<dyn Error>> {
    let repos = parse_env().dir;
    let repos = fs::read_dir(repos)?
        .map(|item| item.unwrap()).into_iter()
        .filter(|item| item.path().is_dir())
        .map(|item| { item.path().file_name().unwrap().to_str().unwrap().to_string()
    }).filter(|path| { !path.contains(".") })
        .collect::<Vec<String>>();
    Ok(repos)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_local_repos() {
        let actuales = local_repos().unwrap();
        for i in actuales.iter() {
            println!("{}",i);
        }
        assert_eq!(actuales.len(),32)
    }
}
