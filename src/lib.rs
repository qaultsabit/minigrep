use std::{env, error::Error, fs};

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn config_build_with_valid_args() {
        let args = vec![
            "program".to_string(),
            "query".to_string(),
            "file.txt".to_string(),
        ];
        let config = Config::build(&args).unwrap();

        assert_eq!(config.query, "query");
        assert_eq!(config.file_path, "file.txt");
        assert!(!config.ignore_case);
    }

    #[test]
    fn config_build_with_not_enough_args() {
        let args = vec!["program".to_string(), "query".to_string()];
        let config = Config::build(&args);

        assert!(config.is_err());
        assert_eq!(config.unwrap_err(), "not enough arguments");
    }

    #[test]
    fn config_build_with_ignore_case() {
        std::env::set_var("IGNORE_CASE", "1");
        let args = vec![
            "program".to_string(),
            "query".to_string(),
            "file.txt".to_string(),
        ];
        let config = Config::build(&args).unwrap();

        assert!(config.ignore_case);
        std::env::remove_var("IGNORE_CASE");
    }

    #[test]
    fn search_finds_exact_matches() {
        let query = "fast";
        let contents = "
Rust is fast,
and memory-efficient.
with zero-cost abstractions.
";
        let results = search(query, contents);

        assert_eq!(results, vec!["Rust is fast,"]);
    }

    #[test]
    fn search_does_not_find_non_matching_lines() {
        let query = "slow";
        let contents = "
Rust is fast,
and memory-efficient.
with zero-cost abstractions.
";
        let results = search(query, contents);

        assert!(results.is_empty());
    }

    #[test]
    fn search_case_insensitive_finds_matches() {
        let query = "rUsT";
        let contents = "
Rust is fast,
and memory-efficient.
with zero-cost abstractions.
";
        let results = search_case_insensitive(query, contents);

        assert_eq!(results, vec!["Rust is fast,"]);
    }

    #[test]
    fn search_case_insensitive_handles_no_matches() {
        let query = "python";
        let contents = "
Rust is fast,
and memory-efficient.
with zero-cost abstractions.
";
        let results = search_case_insensitive(query, contents);

        assert!(results.is_empty());
    }

    #[test]
    fn search_case_insensitive_finds_multiple_matches() {
        let query = "is";
        let contents = "
Rust is fast,
and memory-efficient.
It IS amazing.
";
        let results = search_case_insensitive(query, contents);

        assert_eq!(results, vec!["Rust is fast,", "It IS amazing."]);
    }
}
