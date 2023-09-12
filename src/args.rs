use clap::{Parser, ValueEnum};

use crate::errors::TextToUrlErrors;

#[derive(ValueEnum, Debug, Clone, PartialEq, Eq)]
pub enum OutputTypes {
    Lines,
    First,
}

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short = 'k', long, default_value_t = false)]
    pub ok: bool,

    #[arg(short, long, default_value_t = false)]
    pub clip: bool,

    #[arg(short = 'o', long, value_enum, default_value_t = OutputTypes::First)]
    pub out: OutputTypes,
}

pub fn parse() -> Result<Args, TextToUrlErrors> {
    let args = Args::parse();

    if args.clip && args.out != OutputTypes::First {
        Err(TextToUrlErrors::ArgsIncompatible(
            "cannot use --clip when not selecting the first match".into(),
        ))
    } else {
        Ok(args)
    }
}
