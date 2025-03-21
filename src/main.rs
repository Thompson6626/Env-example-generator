use clap::ArgMatches;
use crate::argc::argc_app;

pub use self::error::{Error, Result};

mod argc;
mod error;

// Current directory
const DIR: &str = "./";

struct Options{
    input_file: String,
    output_file: String,
    keep_comments: bool
}

impl Options {
    fn from_argc(argc: ArgMatches) -> Result<Options>{

        let input_file = argc.get_one::<String>("input").unwrap().to_owned();

        let output_file = argc.get_one::<String>("output").unwrap().to_owned();

        let keep_comments = argc.get_flag("keep-comments");

        Ok(
            Options{
                input_file,
                output_file,
                keep_comments
            }
        )
    }
}

fn main() {
    let argc = argc_app().get_matches();

    // Transforms ArgMatches to Options struct
    let options = match Options::from_argc(argc) {
        Ok(options) => options,
        Err(ex) => {
            println!("ERROR parsing input {}", ex);
            return;
        }
    };

    match exec(options) {
        Ok(_) => (),
        Err(ex) => {
            println!("ERROR - {}", ex);
        }
    }

}

fn exec(options: Options) -> Result<()> {

}