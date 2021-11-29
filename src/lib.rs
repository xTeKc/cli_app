use std::fs;
use std::error::Error;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("With text:\n{}", contents);

    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
    let query = args[1].clone();
    let filename = args[2].clone();

    Ok(Config { query, filename })
}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contetns = "\
    Rust:
    safe, fast, productive.
    Pick three.";
        
        assert_eq!(vec!["safe, fast, productive."], search(query, contetns));
    }
}