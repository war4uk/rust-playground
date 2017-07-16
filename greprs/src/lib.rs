use std::fs::File;
use std::io::prelude::*;

pub fn run(config: Config) -> Result<(), Box<std::error::Error>> {
    let mut file = File::open(&config.filename).expect("not found");

    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    for line in search(&config.query, &contents) {
        println!("{}", line);
    }
    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str>  {
        args.next();

        let query = match args.next() {      
            Some(arg) => arg,
            None => return Err("Cannot get query parameter")
        }; 
        
        let filename = match args.next() {      
            Some(arg) => arg,
            None => return Err("Cannot get filename parameter")
        };
      
        Ok(Config {
            query: query,
            filename: filename
        })
    }
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines().filter(|ln| ln.contains(query)).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }
}