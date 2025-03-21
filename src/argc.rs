use clap::{crate_version, Arg, Command};

pub fn argc_app() -> Command {
    Command::new("genv")
        .version(crate_version!())
        .about("Generate and/or update a .env.example from a .env file")
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .help("Path to the .env file")
                .default_value(".env")
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .help("Path to the .env.example file")
                .default_value(".env.example")
        )
        .arg(
            Arg::new("keep-comments")
                .long("keep-comments")
                .help("Keep comments from .env in the .env.example file")
                .action(clap::ArgAction::SetTrue),
        )
}
