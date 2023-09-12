use thiserror::Error;

#[derive(Error, Debug)]
pub enum TextToUrlErrors {
    #[error("IO error")]
    IO(#[from] std::io::Error),

    #[error("failed to initialise clipboard provider")]
    ClipboardContext,

    #[error("set clipboard")]
    SetClipboardContent,

    #[error("incompatible args: {0}")]
    ArgsIncompatible(String),
}
