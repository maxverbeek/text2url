mod args;
mod errors;

use is_url::is_url;
use std::io::BufRead;

use cli_clipboard::{ClipboardContext, ClipboardProvider};

use errors::TextToUrlErrors as E;

type Res = Result<(), E>;

fn main() -> Res {
    println!("{}", E::ClipboardContext);

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
        std::process::exit(1)
    }

    if args.clip {
        return set_clipboard(urls.next().expect("asserted is_none earlier"));
    }

    for url in urls {
        println!("{}", url);
    }

    Ok(())
}

fn set_clipboard(url: &str) -> Res {
    let mut ctx = ClipboardContext::new().map_err(|_| E::ClipboardContext)?;

    ctx.set_contents(url.to_owned())
        .map_err(|_| E::SetClipboardContent)?;

    Ok(())
}
