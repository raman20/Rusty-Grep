use std::env;
use rustygrep::Config;
use rustygrep::run;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config: Config = Config::build(&args).unwrap_or_else(|err: &str| {
        println!("Problem parsing arguments: {err}");
        std::process::exit(1);
    });

    if let Err(e) = run(config) {
        println!("Application Error {}", e);
        std::process::exit(1);
    }
}