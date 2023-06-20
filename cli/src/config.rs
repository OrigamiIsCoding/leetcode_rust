use crate::parser::Args;
use anyhow::{Error, Ok, Result};
use std::path::PathBuf;
use tera::Tera;

pub struct Config {
    /// problem name
    pub name: String,
    /// problem id
    pub id: usize,
    /// problem url
    pub url: String,
    pub tera: Tera,
    /// output dir
    pub output_dir: String,
    /// template name , `solution_{lang}.jinja`
    pub solution_template_name: String,
}

impl Config {
    pub fn solution_path(&self) -> PathBuf {
        PathBuf::new().join(&self.output_dir).join(format!(
            "s{number:>0width$}_{}.rs",
            &self.name,
            number = self.id,
            width = 4
        ))
    }
}

fn process_url(url: &String) -> Result<String> {
    let paths: Vec<_> = url.split('/').filter(|s| !s.is_empty()).collect();
    let index = paths
        .iter()
        .position(|path| path.eq_ignore_ascii_case("problems"))
        .unwrap_or_else(|| panic!("could't not find problem name from url {}", &url));

    Ok(paths[index + 1].replace('-', "_"))
}

impl TryFrom<Args> for Config {
    type Error = Error;

    fn try_from(args: Args) -> std::result::Result<Self, Self::Error> {
        let name = process_url(&args.url)?;

        let tera = Tera::new(&args.template_path)?;

        Ok(Config {
            name,
            tera,
            id: args.id,
            url: args.url,
            solution_template_name: format!("solution_{}.jinja", &args.lang),
            output_dir: args.output_dir,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_url() {
        let actual =
            process_url(&"https://leetcode.cn/problems/number-of-closed-islands/".into()).unwrap();

        assert_eq!("number_of_closed_islands".to_string(), actual);
    }
}
