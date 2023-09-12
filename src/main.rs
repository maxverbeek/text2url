use is_url::is_url;
use std::io::BufRead;

use cli_clipboard::{ClipboardContext, ClipboardProvider};

use thiserror::Error;

#[derive(Error, Debug)]
enum TextToUrlErrors {
    #[error("IO error")]
    IO(#[from] std::io::Error),

    #[error("failed to initialise clipboard provider")]
    ClipboardContext,

    #[error("set clipboard")]
    SetClipboardContent,
}

fn main() -> Result<(), TextToUrlErrors> {
    let lines: Vec<String> = std::io::stdin()
        .lock()
        .lines()
        .collect::<std::io::Result<Vec<_>>>()
        .map_err(|e| TextToUrlErrors::IO(e))?;

    let first_url = lines
        .iter()
        .flat_map(|l| l.split_whitespace())
        .find(|w| is_url(w));

    if let Some(url) = first_url {
        set_clipboard(url)?
    }

    // TODO: return 1 optionally, implement flags for this
    Ok(())
}

fn set_clipboard(url: &str) -> Result<(), TextToUrlErrors> {
    let mut ctx = ClipboardContext::new().map_err(|_| TextToUrlErrors::ClipboardContext)?;

    ctx.set_contents(url.to_owned())
        .map_err(|_| TextToUrlErrors::SetClipboardContent)?;

    Ok(())
}
