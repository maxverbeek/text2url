mod args;
mod errors;

use is_url::is_url;
use std::{io::BufRead, process::exit};

use cli_clipboard::{ClipboardContext, ClipboardProvider};

use errors::TextToUrlErrors as E;

type Res = Result<(), E>;

fn main() -> Res {
    let args = args::parse()?;

    let lines: Vec<String> = std::io::stdin()
        .lock()
        .lines()
        .collect::<std::io::Result<Vec<_>>>()
        .map_err(|e| E::IO(e))?;

    let mut urls = lines
        .iter()
        .flat_map(|l| l.split_whitespace())
        .filter(|w| is_url(w))
        .peekable();

    if !args.ok && urls.peek().is_none() {
        exit(1)
    }

    if let Some(url) = urls.peek() {
        if args.clip {
            set_clipboard(url)?;
        }
    }

    match args.out {
        args::OutputTypes::First => urls.take(1).for_each(|u| println!("{}", u)),
        args::OutputTypes::Lines => urls.for_each(|u| println!("{}", u)),
        args::OutputTypes::Tee => {
            lines.iter().for_each(|l| println!("{}", l));
        }
    }

    Ok(())
}

fn set_clipboard(url: &str) -> Res {
    let mut ctx = ClipboardContext::new().map_err(|_| E::ClipboardContext)?;

    ctx.set_contents(url.to_owned())
        .map_err(|_| E::SetClipboardContent)?;

    Ok(())
}
