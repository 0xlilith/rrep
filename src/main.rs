use std::env;
use std::error::Error;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::parse(&args).unwrap_or_else(|err| {
        println!("<error>: {}",err);
        process::exit(1);
    });

    println!("Searching for {}",config.search);
    if let Err(err) = run(config) {
        println!("<error>: {}",err);
        process::exit(1);
    };
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let buf = fs::read_to_string(config.file)?;
    println!("{}", buf);

    Ok(())
}

struct Config {
    search: String,
    file: String,
}

impl Config {
    fn parse(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("arguments not satisfied");
        }
        let search = args[1].clone();
        let file = args[2].clone();
    
        Ok(Config {search, file})
    }
}

