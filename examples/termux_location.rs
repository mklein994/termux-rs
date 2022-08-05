#![cfg(feature = "example")]

use clap::Parser;
use termux_rs::termux_location::*;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::parse();
    let location = TermuxLocation::termux_location(opt.provider, opt.request)?;
    print!("{location}");
    Ok(())
}

#[derive(Parser)]
pub struct Opt {
    #[clap(short, long, value_enum, default_value_t)]
    provider: Provider,

    #[clap(short, long, value_enum, default_value_t)]
    request: Request,
}
