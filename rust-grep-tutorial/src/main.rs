use rust_grep_tutorial::Config;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for '{}'", config.query);
    println!("In File for '{}'", config.file_path);
    // println!("Config: '{}'", config);

    if let Err(e) = rust_grep_tutorial::run(config) {
        eprintln!("Application Error: {e}");
        process::exit(1);
    }
}
