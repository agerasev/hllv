//mod repo;
mod api;
mod cli;

use std::{
    env,
    process::exit,
};

fn main() {
    let cwd = env::current_dir().unwrap();
    let args: Vec<String> = env::args().collect();
    let args_ref: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
    exit(cli::run::<api::MainApi>(&cwd, &args_ref));
}
