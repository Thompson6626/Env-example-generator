use derive_more::{Display, From};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, From, Display)]
pub enum Error {
    #[display("Input file '{}' not found", _0)]
    InputFileNotFound(String),

    #[display("Output file '{}' could not be created", _0)]
    OutputFileCreationFailed(String),

    #[display("Failed to read .env file '{}'", _0)]
    EnvFileReadError(String),

    #[display("Failed to write to .env.example file '{}'", _0)]
    EnvFileWriteError(String),

    #[display("Invalid command-line argument: {}", _0)]
    InvalidArgument(String),

    #[display("Unexpected error: {}", _0)]
    Unknown(String),

    // External Errors
    #[from]
    Io(std::io::Error),
    #[from]
    Clap(clap::Error),
}

impl std::error::Error for Error {}
