use std::{fs::File, io::Write, path::Path};

use clap::Parser;
use tera::{Context, Tera};

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    id: i32,
    /// Template Path
    #[arg(short, long, default_value_t = String::from("templates/*"))]
    template_path: String,
    #[arg(short, long)]
    name: String,
}

fn main() {
    let args = Args::parse();
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
            &args.name,
            number = args.id,
            width = 4
        )))
        .unwrap();
    file.write_all(rendered.as_bytes()).unwrap();
}
