use clap::{crate_version, Arg, Command};

pub fn argc_app() -> Command {

    Command::new("genv")
        .version(crate_version!())
        .about("Generate and/or update a .env.example from a .env file")
        .arg(
            Arg::new("input")
                .short('i') // -i
                .long("input") // --input
                .help("Path to the .env file")
                .default_value(".env")
        )
        .arg(
            Arg::new("output")
                .short('o') // -o
                .long("output")// --output
                .help("Path to the .env.example file")
                .default_value(".env.example")
        )
        .arg(
            Arg::new("keep-comments")
                .long("keep-comments") // --keep-comments
                .help("Keep comments from .env in the .env.example file")
                .action(clap::ArgAction::SetTrue),
        ).arg(
            Arg::new("overwrite")
                .short('w') // -w
                .long("overwrite") // --overwrite
                .help("Whether to overwrite the .env.example file or update it")
                .action(clap::ArgAction::SetTrue)
        )
}
