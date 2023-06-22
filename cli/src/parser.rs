use clap::Parser;

/// Leetcode command line interface
#[derive(Parser, Debug)]
#[command(
    author = "Origami",
    version = "0.0.1",
    about = "generate solution code."
)]
pub struct Args {
    /// problem id
    #[arg(short, long)]
    pub id: usize,
    /// template Path
    #[arg(short = 'p', long, default_value_t = String::from("templates/*"))]
    pub template_path: String,
    /// problem url
    #[arg(short, long)]
    pub url: String,
    /// code output solution dir
    #[arg(short, long, default_value_t = String::from("solution/src/solution"))]
    pub output_dir: String,
    /// lang
    #[arg(short, long, default_value_t = String::from("rust"))]
    pub lang: String,
    /// series
    /// - solution
    /// - lcp
    #[arg(short, long, default_value_t = String::from("solution"))]
    pub series: String,
}
