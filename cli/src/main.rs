use clap::Parser;
use cli::{parser::Args, runner};

fn main() {
    let args = Args::parse();
    if let Err(err) = runner::run(args) {
        eprintln!("{:?}", err);
    }
}
