use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

use crate::{config::Config, parser::Args};
use anyhow::{Ok, Result};
use tera::Context;

pub fn run(args: Args) -> Result<()> {
    let mut config = Config::try_from(args)?;

    generate_solution_file(&mut config)?;
    generate_mod_file(&config)?;

    Ok(())
}

fn generate_solution_file(config: &mut Config) -> Result<()> {
    let mut ctx = Context::new();

    ctx.insert("url", &config.url);

    let content = config.tera.render(&config.solution_template_name, &ctx)?;

    let mut file = File::options()
        .create(true)
        .write(true)
        .open(config.solution_path())?;

    file.write_all(content.as_bytes())?;

    Ok(())
}

fn generate_mod_file(config: &Config) -> Result<()> {
    let solution_dir = PathBuf::from(&config.output_dir);

    let mut solutions: Vec<_> = fs::read_dir(&solution_dir)?
        .map(|entry| entry.unwrap().file_name().into_string().unwrap())
        .filter(|name| name.contains('_'))
        .map(|name| {
            let number = name.split_once('_').unwrap().0[1..].parse::<i32>().unwrap();
            let name = name.split_once('.').unwrap().0.to_string();
            (number, name)
        })
        .collect();

    solutions.sort_by(|a, b| a.0.cmp(&b.0));

    let mod_content = solutions
        .into_iter()
        .map(|(_, name)| format!("mod {};", name))
        .collect::<Vec<_>>()
        .join("\n");

    let mut file = File::options()
        .write(true)
        .open(solution_dir.join("mod.rs"))?;

    file.write_all(mod_content.as_bytes())?;

    Ok(())
}
