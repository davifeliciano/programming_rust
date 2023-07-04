use regex::Regex;
use std::{env, fs};
use text_colorizer::*;

#[derive(Debug)]
struct Arguments {
    pattern: String,
    replacement: String,
    filename: String,
    output: String,
}

fn main() {
    let args = parse_args();

    let data = match fs::read_to_string(&args.filename) {
        Ok(v) => v,
        Err(e) => {
            eprintln!(
                "{} failed to read from file '{}': {:?}",
                "Error:".red().bold(),
                args.filename,
                e
            );
            std::process::exit(1)
        }
    };

    let replaced_data = match replace(&args.pattern, &args.replacement, &data) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{} failed to replace text: {:?}", "Error:".red().bold(), e);
            std::process::exit(1)
        }
    };

    match fs::write(&args.output, &replaced_data) {
        Ok(_) => {}
        Err(e) => {
            eprintln!(
                "{} failed to write to file '{}': {:?}",
                "Error:".red().bold(),
                args.filename,
                e
            );
            std::process::exit(1);
        }
    };
}

fn print_usage() {
    let program_name = env::args().nth(0).unwrap();
    eprintln!(
        "{} - replace occurrences of pattern and save it in another file",
        &program_name.green()
    );
    eprintln!(
        "Usage: {} <pattern> <replacement_pattern> <input_file> <output_file>",
        &program_name.green()
    );
}

fn parse_args() -> Arguments {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 4 {
        print_usage();
        eprintln!(
            "{} wrong number of arguments: expected 4, got {}",
            "Error:".red().bold(),
            args.len()
        );
        std::process::exit(1);
    }

    Arguments {
        pattern: args[0].clone(),
        replacement: args[1].clone(),
        filename: args[2].clone(),
        output: args[3].clone(),
    }
}

fn replace(pattern: &str, replacement: &str, text: &str) -> Result<String, regex::Error> {
    let regex = Regex::new(pattern)?;
    Ok(regex.replace_all(text, replacement).to_string())
}
