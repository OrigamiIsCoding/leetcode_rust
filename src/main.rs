use std::{
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};

use clap::Parser;
use tera::{Context, Tera};

/// Generate leetcode solution rust file
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Task Type
    #[arg(long)]
    task: String,
    /// Problem id
    #[arg(short, long)]
    id: i32,
    /// Template Path
    #[arg(short = 'p', long, default_value_t = String::from("templates/*"))]
    template_path: String,
    /// Problem Title
    #[arg(short = 't', long)]
    title: String,
}

fn main() {
    let args = Args::parse();
    match args.task.as_str() {
        "Solution" => {
            let tera = match Tera::new(&args.template_path) {
                Err(e) => {
                    panic!("Parsing error(s): {}", e)
                }
                Ok(t) => t,
            };
            let ctx = Context::new();
            let rendered = tera.render("solution_template.html", &ctx).unwrap();
            let mut file = File::options()
                .create(true)
                .write(true)
                .open(Path::new("./").join("src").join("solution").join(format!(
                    "s{number:>0width$}_{}.rs",
                    &args.title.replace("-", "_"),
                    number = args.id,
                    width = 4
                )))
                .unwrap();
            file.write_all(rendered.as_bytes()).unwrap();

        },
        "Mod" => {
            let mut solution_files = Vec::new();
            for entry in fs::read_dir(Path::new("./").join("src").join("solution")).unwrap() {
                let entry = entry.unwrap();
                let file_name = entry.file_name().into_string().unwrap();
                if file_name.starts_with("s") && file_name.contains("_") {
                    solution_files.push(file_name);
                }
            }

            let mut solutions: Vec<_> = solution_files
                .into_iter()
                .map(|file_name| {
                    let parts: Vec<_> = file_name.split("_").collect();
                    let number = parts[0][1..].parse::<i32>().unwrap();
                    let mut name = PathBuf::from(parts[1..].join("_").to_string());
                    name.set_extension("");
                    (number, name.to_str().unwrap().to_string())
                })
                .collect();

            solutions.sort_by(|a, b| a.0.cmp(&b.0));

            let mod_content = solutions
                .into_iter()
                .map(|(id, name)| {
                    format!("mod s{number:>0width$}_{};", name, number = id, width = 4)
                })
                .collect::<Vec<_>>()
                .join("\n");
            
            let mut file = File::options()
                .create(true)
                .write(true)
                .open(Path::new("./").join("src").join("solution").join("mod.rs"))
                .unwrap();
            file.write_all(mod_content.as_bytes()).unwrap();
        },
        _ => println!("Unsupported Task Type")
    }
}
