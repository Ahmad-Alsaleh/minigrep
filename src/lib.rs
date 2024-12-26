use std::{env, error::Error, fs};

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Self, String> {
        if args.len() != 3 {
            return Result::Err(format!(
                "Invalid number of arguments. Usage: {} <QUERY> <FILE_PATH>",
                args[0]
            ));
        }
        Ok(Self {
            query: args[1].clone(),
            file_path: args[2].clone(),
            ignore_case: env::var("IGNORE_CASE").is_ok_and(|value| value == "1"),
        })
    }
}

pub fn search<'a>(query: &str, content: &'a str, ignore_case: bool) -> Vec<&'a str> {
    let query = if ignore_case {
        &query.to_lowercase()
    } else {
        query
    };
    content
        .lines()
        .filter(|&line| {
            let line = if ignore_case {
                &line.to_lowercase()
            } else {
                line
            };
            line.contains(query)
        })
        .collect()
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(&config.file_path)?;
    for line in search(&config.query, &content, config.ignore_case) {
        println!("{line}");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.";
        let result = search(query, content, false);
        assert_eq!(result, vec!["safe, fast, productive."]);
    }

    #[test]
    fn ignore_case() {
        let query = "How";
        let content = "\
Hello
How are you?
how am I?!
yes!";
        let result = search(query, content, true);
        assert_eq!(result, vec!["How are you?", "how am I?!"]);

        let result = search(query, content, false);
        assert_eq!(result, vec!["How are you?"]);
    }
}
