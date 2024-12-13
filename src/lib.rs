use std::{env, error::Error, fs};

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next(); // Skip the program name
        let query = args.next().ok_or("Didn't get a query string")?;
        let file_path = args.next().ok_or("Didn't get a file path")?;
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

    let search_fn = if config.ignore_case {
        search_case_insensitive
    } else {
        search
    };

    search_fn(&config.query, &contents)
        .into_iter()
        .for_each(|line| println!("{line}"));

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();

    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_build_with_valid_args() {
        let args = vec![
            "program".to_string(),
            "query".to_string(),
            "file.txt".to_string(),
        ];
        let config = Config::build(args.into_iter()).unwrap();

        assert_eq!(config.query, "query");
        assert_eq!(config.file_path, "file.txt");
        assert!(!config.ignore_case);
    }

    #[test]
    fn config_build_with_not_enough_args() {
        let args = vec!["program".to_string(), "query".to_string()];
        let config = Config::build(args.into_iter());

        assert!(config.is_err());
        assert_eq!(config.unwrap_err(), "Didn't get a file path");
    }

    #[test]
    fn config_build_with_ignore_case() {
        env::set_var("IGNORE_CASE", "1");
        let args = vec![
            "program".to_string(),
            "query".to_string(),
            "file.txt".to_string(),
        ];
        let config = Config::build(args.into_iter()).unwrap();

        assert!(config.ignore_case);
        env::remove_var("IGNORE_CASE");
    }

    #[test]
    fn search_finds_exact_matches() {
        let query = "fast";
        let contents = "Rust is fast,\nand memory-efficient.\nwith zero-cost abstractions.\n";
        let results = search(query, contents);

        assert_eq!(results, vec!["Rust is fast,"]);
    }

    #[test]
    fn search_does_not_find_non_matching_lines() {
        let query = "slow";
        let contents = "Rust is fast,\nand memory-efficient.\nwith zero-cost abstractions.\n";
        let results = search(query, contents);

        assert!(results.is_empty());
    }

    #[test]
    fn search_case_insensitive_finds_matches() {
        let query = "rUsT";
        let contents = "Rust is fast,\nand memory-efficient.\nwith zero-cost abstractions.\n";
        let results = search_case_insensitive(query, contents);

        assert_eq!(results, vec!["Rust is fast,"]);
    }

    #[test]
    fn search_case_insensitive_handles_no_matches() {
        let query = "python";
        let contents = "Rust is fast,\nand memory-efficient.\nwith zero-cost abstractions.\n";
        let results = search_case_insensitive(query, contents);

        assert!(results.is_empty());
    }

    #[test]
    fn search_case_insensitive_finds_multiple_matches() {
        let query = "is";
        let contents = "Rust is fast,\nand memory-efficient.\nIt IS amazing.\n";
        let results = search_case_insensitive(query, contents);

        assert_eq!(results, vec!["Rust is fast,", "It IS amazing."]);
    }
}
