use thiserror::Error;

#[derive(Error, Debug)]
pub enum TextToUrlErrors {
    #[error("IO error")]
    IO(#[from] std::io::Error),

    #[error("spawning set clipboard process")]
    SetClipboardSpawn(std::io::Error),

    #[error("writing stdin to set clipboard process")]
    SetClipboardPipe(std::io::Error),

    #[error("incompatible args: {0}")]
    ArgsIncompatible(String),
}
