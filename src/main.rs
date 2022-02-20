/*
project: rrep
author: 0xlilith
*/
use std::env;
use std::process;

use rrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::parse(&args).unwrap_or_else(|err| {
        eprintln!("<error>: {}",err);
        process::exit(1);
    });

    if let Err(err) = rrep::run(config) {
        eprintln!("<error>: {}",err);
        process::exit(1);
    };
}
