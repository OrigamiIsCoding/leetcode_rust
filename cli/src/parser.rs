use clap::Parser;

/// Leetcode command line interface
#[derive(Parser, Debug)]
#[command(
    author = "Origami",
    version = "0.0.1",
    about = "generate solution code."
)]
pub struct Args {
    /// Problem id
    #[arg(short, long)]
    pub id: usize,
    /// Template path
    #[arg(short = 'p', long, default_value_t = String::from("templates/*"))]
    pub template_path: String,
    /// Problem url
    #[arg(short, long)]
    pub url: String,
    /// Code output solution dir
    #[arg(short, long, default_value_t = String::from("solution/src/solution"))]
    pub output_dir: String,
    /// Lang
    #[arg(short, long, default_value_t = String::from("rust"))]
    pub lang: String,
    /// Problem series, such as
    /// - solution
    /// - lcp
    /// - interview : { pattern: "面试题 xx.xx problem_name" }
    #[arg(short, long, default_value_t = String::from("solution"))]
    pub series: String,
}
