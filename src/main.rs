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
    }).filter(|path| { !path.starts_with(".") })
        .collect::<Vec<String>>();
    Ok(repos)
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::fs::{File, remove_dir_all, create_dir_all};
    use std::io::Write;

    fn test_dir_with_files(sub_path: &str) -> String {
        let ret = format!("{}{}",get_test_dir(),sub_path);
        fs::create_dir_all(Path::new(&ret)).unwrap();
        let string = format!("{}/1.txt", ret.as_str());
        File::create(Path::new(&string)).map( |mut x| {
            x.write_all(b"da").unwrap();
        }).unwrap();
        let string = format!("{}/2.txt", ret.as_str());
        File::create(Path::new(&string)).map(|mut x| {
            x.write_all(b"bananush").unwrap();
        }).unwrap();
        ret
    }

    fn get_test_dir() -> String {
        let da = format!("{temp}testush", temp = env::temp_dir().as_path().display().to_string());
        println!("testdit ---> {}",da);
        da
    }

    fn clean() {
        remove_dir_all(get_test_dir()).unwrap_or_else(|_| println!("oops"));
    }

    #[test]
    fn count_only_repos() {
        clean();
        test_dir_with_files("/banana");
        env::set_var("REPOS_DIR", get_test_dir());
        let actual = local_repos().unwrap();
        for i in actual.iter() {
            println!("{}",i);
        }
        assert_eq!(actual.len(),1);
    }

    #[test]
    fn ignore_non_dir() {
        clean();
        test_dir_with_files("");
        env::set_var("REPOS_DIR", get_test_dir());
        let repos = parse_env().dir;
        println!("reposss {}", repos);

        let actual = local_repos().unwrap();
        for i in actual.iter() {
            println!("{}",i);
        }
        assert_eq!(actual.len(),0);

    }
}
