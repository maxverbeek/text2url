mod args;
mod errors;

use is_url::is_url;
use std::{
    io::{BufRead, Write},
    process::{exit, Command, Stdio},
};

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

    if !args.ok && args.out != args::OutputTypes::Tee && urls.peek().is_none() {
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
    let mut proc = Command::new("xclip")
        .args(["-selection", "clipboard", "-in"])
        .stdin(Stdio::piped())
        .spawn()
        .map_err(|e| E::SetClipboardSpawn(e))?;

    let child_stdin = proc.stdin.as_mut().unwrap();
    child_stdin
        .write_fmt(format_args!("{}", url))
        .map_err(|e| E::SetClipboardPipe(e))?;

    proc.wait()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::set_clipboard;

    #[test]
    fn test_clipboard() {
        set_clipboard("testing").unwrap();
    }
}
