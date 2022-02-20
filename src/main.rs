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
        println!("<error>: {}",err);
        process::exit(1);
    });

    println!("Searching for {}",config.search);
    if let Err(err) = rrep::run(config) {
        println!("<error>: {}",err);
        process::exit(1);
    };
}
