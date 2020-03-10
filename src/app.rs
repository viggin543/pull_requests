use clap::{self, crate_authors, crate_version, App, AppSettings, Arg, Shell};
use std::env;

const USAGE: &str = "\
prs -alfred
prs
";
const TEMPLATE: &str = "\
{bin} {version}
{author}
{about}

USAGE:{usage}

ARGS:
{positionals}

OPTIONS:
{unified}";

pub fn app() -> App<'static, 'static> {
    App::new("pull-requests")
        .author(crate_authors!())
        .version(crate_version!())
        .long_version("0.1")
        .about("list all your pull requests within your org repos")
        .max_term_width(100)
        .setting(AppSettings::UnifiedHelpMessage)
        .setting(AppSettings::AllArgsOverrideSelf)
        .usage(USAGE)
        .template(TEMPLATE)
        .help_message("Prints help information. Use --help for more details.")
        .arg(alfred_flag())
        .arg(configure_flag())
}

fn alfred_flag() -> Arg<'static, 'static> {
    const SHORT: &str = "Show NUM lines after each match.";
    const LONG: &str = "\
alfred json output format
{ 'title': 'http://github/user/repo', 'subtitle':'user', 'arg':'url of pull request' }
";

    Arg::with_name("alfred")
        .short("a")
        .default_value("false")
        .required(false)
        .help(SHORT)
        .long_help(LONG)
}

fn configure_flag() -> Arg<'static, 'static> {
    Arg::with_name("configure")
        .default_value("false")
        .required(false)
        .help("configure the tool")
}

#[derive(Debug, Eq, PartialEq)]
pub struct Config {
    pub dir: String,
    pub org: String,
    pub pass: String,
    pub user: String,
}

pub fn parse_env() -> Config {
    let dir = env::var("REPOS_DIR")
        .expect("REPOS_DIR env var missing")
        .to_string();
    let org = env::var("GITHUB_ORG")
        .expect("GITHUB_ORG env var missing")
        .to_string();
    let pass = env::var("GITHUB_PASS")
        .expect("GITHUB_PASS env var missing")
        .to_string();
    let user = env::var("GITHUB_USER")
        .expect("GITHUB_USER env var missing")
        .to_string();
    Config {
        dir,
        org,
        pass,
        user,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_alfred_alg() {
        assert_eq!("false", app().get_matches().value_of("alfred").unwrap())
    }

    #[test]
    #[should_panic]
    fn parse_env_test() {
        env::remove_var("GITHUB_ORG");
        let conf = parse_env();
        println!("conf: {:?}", conf);
    }

    #[test]
    fn can_parse_arg() {
        env::set_var("GITHUB_ORG", "org");
        env::set_var("GITHUB_PASS", "pass");
        env::set_var("GITHUB_USER", "user");
        env::set_var("REPOS_DIR", "dir");
        let actual = parse_env();
        assert_eq!(
            Config {
                dir: "dir".to_string(),
                org: "org".to_string(),
                pass: "pass".to_string(),
                user: "user".to_string()
            },
            actual
        )
    }
}
