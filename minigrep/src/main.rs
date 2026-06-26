use std::env;
use std::error::Error;
use std::fs;
use std::process;

use minigrep::{search, search_case_insensitive};

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Args: {:#?}", args);

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Config: {:#?}", config);
    println!("Searching for: {}", config.query);
    println!("In file      : {:?}", config.file_paths);

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    for file_path in &config.file_paths {
        // Controleer of het pad een directory is
        match fs::metadata(file_path) {
            Ok(metadata) if metadata.is_dir() => {
                eprintln!("Error: '{}' is a directory, skipping", file_path);
                continue;
            }
            Ok(_) => {} // Bestandsmetadata OK, doorgaan
            Err(e) => {
                eprintln!("Error accessing '{}': {}", file_path, e);
                continue;
            }
        }

        let bytes = match fs::read(file_path) {
            Ok(b) => b,
            Err(e) => {
                eprintln!("Error reading {}: {}", file_path, e);
                continue;
            }
        };

        // Detecteer binaire bestanden (bevat null-byte)
        let is_binary = bytes.contains(&0);

        // Converteer altijd met lossy (werkt voor tekst én binair)
        let contents = String::from_utf8_lossy(&bytes).into_owned();

        let results = if config.ignore_case {
            search_case_insensitive(&config.query, &contents)
        } else {
            search(&config.query, &contents)
        };

        for line in results {
            if is_binary {
                // Limiteer tot 20 karakters voor binaire bestanden
                let limited = if line.chars().count() > 20 {
                    format!("{}…", line.chars().take(20).collect::<String>())
                } else {
                    line.to_string()
                };
                println!("{}:{}", file_path, limited);
            } else {
                println!("{}:{}", file_path, line);
            }
        }
    }
    Ok(())
}

#[derive(Debug)]
struct Config {
    query: String,
    file_paths: Vec<String>,
    pub ignore_case: bool,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        // Tel --ignore-case flags en verzamel positionele argumenten
        let mut ignore_case_count = 0;
        let mut non_flag_args = Vec::new();

        for arg in &args[1..] {
            if arg == "--ignore-case" {
                ignore_case_count += 1;
            } else if arg.starts_with("--") {
                eprintln!("Error: Unknown flag '{}'", arg);
                std::process::exit(1);
            } else {
                non_flag_args.push(arg.clone());
            }
        }

        println!("ignore_case: {ignore_case_count}");
        println!("non_flag_args: {:?}", non_flag_args);

        // Valideer argumenten
        if non_flag_args.len() < 2 {
            eprintln!(
                "Usage: {} [--ignore-case] <search-string> <file_path>",
                args[0]
            );
            std::process::exit(1);
        }

        // Controleer environment variable
        let env_ignore_case = env::var_os("IGNORE_CASE").is_some();

        // Waarschuwingen
        if ignore_case_count > 1 {
            eprintln!("Warning: --ignore-case specified multiple times");
        }
        if ignore_case_count > 0 && env_ignore_case {
            eprintln!(
                "Warning: Both --ignore-case flag and IGNORE_CASE environment variable are set. Using the flag."
            );
        }

        // Bepaal case-insensitive modus
        let ignore_case = ignore_case_count > 0 || env_ignore_case;
        let query = non_flag_args[0].clone();
        let file_paths = non_flag_args[1..].to_vec();

        Ok(Config {
            query,
            file_paths,
            ignore_case,
        })
    }
}
