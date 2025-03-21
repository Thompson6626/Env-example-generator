use std::collections::HashSet;
use clap::ArgMatches;
use std::fs;
use std::fs::OpenOptions;
use std::path::Path;
use crate::argc::argc_app;
pub use self::error::{Error, Result};
use std::io::Write;

mod argc;
mod error;

struct Options {
    input_file: String,
    output_file: String,
    keep_comments: bool,
    overwrite: bool
}

impl Options {
    fn from_argc(argc: ArgMatches) -> Result<Options> {

        let input_file = argc
            .get_one::<String>("input")
            .ok_or_else(|| Error::InvalidArgument("Missing input file argument".into()))?
            .to_owned();


        let output_file = argc
            .get_one::<String>("output")
            .ok_or_else(|| Error::InvalidArgument("Missing output file argument".into()))?
            .to_owned();


        let keep_comments = argc.get_flag("keep-comments");

        let overwrite = argc.get_flag("overwrite");

        Ok(Options {
            input_file,
            output_file,
            keep_comments,
            overwrite
        })
    }
}

use std::process;

fn main() {
    let argc = argc_app().get_matches();

    // Parse CLI arguments into Options
    let options = match Options::from_argc(argc) {
        Ok(options) => options,
        Err(ex) => {
            eprintln!("ERROR parsing input: {}", ex);
            process::exit(1);
        }
    };

    // Execute the main program
    if let Err(ex) = exec(options) {
        eprintln!("ERROR - {}", ex);
        process::exit(1);
    }
}


fn exec(options: Options) -> Result<()> {

    let input_path = Path::new(&options.input_file);
    let output_path = Path::new(&options.output_file);

    // Check if the input file exists
    if !input_path.exists() {
        return Err(Error::InputFileNotFound(options.input_file));
    }

    // Read the .env file
    let contents = fs::read_to_string(input_path)
        .map_err(|e| Error::EnvFileReadError(format!("{}: {}", options.input_file, e)))?;

    let processed = if options.overwrite {
        overwrite_env_file(&contents, options.keep_comments)
    } else {
        let example_contents = fs::read_to_string(output_path)
            .map_err(|e | Error::EnvFileReadError(format!("{}: {}", options.output_file, e)))?;

        process_env_file(&contents, &example_contents, options.keep_comments)
    };

    // Write to .env.example
    if options.overwrite {
        // Overwrite the file
        fs::write(output_path, processed)
            .map_err(|_| Error::EnvFileWriteError(options.output_file.clone()))?;
    } else {
        // Append to the file
        let mut file = OpenOptions::new()
            .create(true) // Create the file if it doesn't exist
            .append(true) // Enable appending
            .open(output_path)
            .map_err(|_| Error::EnvFileWriteError(options.output_file.clone()))?;

        writeln!(file, "{}", processed)
            .map_err(|_| Error::EnvFileWriteError(options.output_file.clone()))?;
    }

    println!("✅ Successfully processed '{}' -> '{}'", options.input_file, options.output_file);

    Ok(())
}

/// Parses `.env` content, extracting only variable keys while preserving comments if required.
fn overwrite_env_file(contents: &str, keep_comments: bool) -> String {
    collect_keys(contents,keep_comments)
        .collect::<Vec<_>>()
        .join("\n") // So that they are in new lines
}

fn process_env_file(contents: &str, example_contents: &str, keep_comments: bool) -> String {
    let env_keys: HashSet<String> = collect_keys(contents, keep_comments).collect();
    let example_keys: HashSet<String> = collect_keys(example_contents, keep_comments).collect();

    println!("{:?}", env_keys);
    println!("{:?}", example_keys);

    let difference: Vec<_> = env_keys.difference(&example_keys).cloned().collect();

    println!("{:?}", difference);

    difference.join("\n")
}

fn collect_keys<'a>(content: &'a str, keep_comments: bool) -> impl Iterator<Item = String> + 'a {
    content
        .lines()
        .filter_map(move |line| {
            let trimmed = line.trim();

            if trimmed.is_empty() {
                Some(String::new()) // Keep empty lines
            } else if trimmed.starts_with('#') {
                if keep_comments {
                    Some(line.to_string()) // Preserve comments if enabled
                } else {
                    None // Remove comments if not needed
                }
            } else if let Some((key, _)) = trimmed.split_once('=') {
                Some(key.trim().to_string()) // Store only the key (without `=`)
            } else {
                None // Ignore malformed lines
            }
        })
}

