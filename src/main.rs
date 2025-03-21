use std::collections::HashSet;
use clap::ArgMatches;
use std::fs;
use std::fs::OpenOptions;
use std::path::Path;
use crate::argc::argc_app;
pub use self::error::{Error, Result};
use std::io::Write;
use std::process;

mod argc;
mod error;

struct Options {
    input_file: String,
    output_file: String,
    omit_comments: bool,
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


        let omit_comments = argc.get_flag("omit-comments");

        let overwrite = argc.get_flag("overwrite");

        Ok(Options {
            input_file,
            output_file,
            omit_comments,
            overwrite
        })
    }
}


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

    if !output_path.exists() {
        fs::File::create(&output_path).expect("Failed to create file");
    }

    // Read the .env file
    let contents = fs::read_to_string(input_path)
        .map_err(|e| Error::EnvFileReadError(format!("{}: {}", options.input_file, e)))?;

    let processed = if options.overwrite {
        overwrite_env_file(&contents, options.omit_comments)
    } else {
        let example_contents = fs::read_to_string(output_path)
            .map_err(|e | Error::EnvFileReadError(format!("{}: {}", options.output_file, e)))?;

        process_env_file(&contents, &example_contents, options.omit_comments)
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
fn overwrite_env_file(contents: &str, omit_comments: bool) -> String {
    collect_keys(contents, omit_comments, false).join("\n")
}

fn process_env_file(contents: &str, example_contents: &str, omit_comments: bool) -> String {
    let env_keys: HashSet<_> = collect_keys(contents, omit_comments, false).into_iter().collect();
    let example_keys: HashSet<_> = collect_keys(example_contents, omit_comments, true).into_iter().collect();

    env_keys.difference(&example_keys).cloned().collect::<Vec<_>>().join("\n")
}

fn collect_keys(content: &str, omit_comments: bool, keep_values: bool) -> Vec<String> {
    content
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim();

            match trimmed {
                "" => Some(String::new()), // Keep empty lines
                _ if trimmed.starts_with('#') => (!omit_comments).then(|| line.to_string()), // Keep/remove comments
                _ if keep_values => Some(trimmed.to_string()), // Preserve full line for example file
                _ => trimmed.split_once('=').map(|(key, _)| key.trim().to_string()), // Store only keys
            }
        })
        .collect()
}

