use std::{env, process::exit};

use minigrep::{run, Config};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|error| {
        println!("Problem parsing arguments: {error}");
        exit(1);
    });
    if let Err(e) = run(config) {
        println!("Failed to run {}: {}", args[0], e);
        exit(1);
    }
}
