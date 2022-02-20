/*
rrep library
author: 0xlilith
*/
use std::error::Error;
use std::fs;
use::std::env;
use std::process;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let buf = fs::read_to_string(config.file)?;
    
    let result = if config.case_flag {
        search(&config.search, &buf)
    } else {
        search_cs(&config.search, &buf)
    };

    for line in result {
        println!("{}", line);
    }

    Ok(())
}

pub struct Config {
    pub search: String,
    pub file: String,
    pub case_flag: bool,
}

impl Config {
    pub fn parse(args: &[String]) -> Result<Config, &str> {
        if args[1] == "-h" || args[1] == "-help" {
            menu();
            process::exit(1);
        }
        if args.len() < 3 {
            return Err("arguments not satisfied");
        }
        let search = args[1].clone();
        let file = args[2].clone();
        let case_flag = env::var("CASE").is_err();
        
        Ok(Config {search, file, case_flag})
    }
}

pub fn search<'a>(search: &str, contents: &'a str) -> Vec<&'a str>{
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(&search) {
            results.push(line);
        }
    }

    results
}

pub fn search_cs<'a>(search: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    // let search = search.to_lowercase();
    for line in contents.lines() {
        if line.to_lowercase().contains(&search.to_lowercase()) {
            results.push(line);
        }
    }

    results
}

pub fn menu() {
    println!("<usage>: rrep [search] [filename.extension]");
}