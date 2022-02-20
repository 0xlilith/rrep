/*
rrep library
author: 0xlilith
*/
use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let buf = fs::read_to_string(config.file)?;
    println!("{}", buf);

    Ok(())
}

pub struct Config {
    pub search: String,
    pub file: String,
}

impl Config {
    pub fn parse(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("arguments not satisfied");
        }
        let search = args[1].clone();
        let file = args[2].clone();
    
        Ok(Config {search, file})
    }
}