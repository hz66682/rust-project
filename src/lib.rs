use std::{env, error::Error, fs, result};

pub struct Config {
    query: String,
    filepath: String,
    ignore_case: bool
}
impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filepath = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").map_or(false, |v| v == "1");
        Ok(Config { query, filepath, ignore_case })
    }
}
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.filepath)?;
    for line in search(&config.query, &content, config.ignore_case) {
        println!("{line}");
    }
    // println!("With text:\n{content}");

    Ok(()) 
}
pub fn search<'a>(query: &str, contents: &'a str, flag: bool) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if !flag {
            if line.contains(query) {
                results.push(line);
            }
        } else {
            if line.to_lowercase().contains(&query.to_lowercase()) {
                results.push(line);
            }
        }
    }
    results
}
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn one() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents, false));
    }
    #[test]
    fn two() {
        let query = "RuSt";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
RuSt:good
Trust me.";
        assert_eq!(vec!["Rust:", "RuSt:good", "Trust me."], search(query, contents, true));
        assert_eq!(vec!["RuSt:good"], search(query, contents, false));
    }  
}