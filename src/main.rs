#![feature(iter_intersperse)]

mod encodings;
mod error;
mod language;
mod score;
mod utils;

mod subcommands;
use subcommands::{Subcommand, SubcommandArgs};

use structopt::StructOpt;

use std::error::Error;

fn main() {
    let args = SubcommandArgs::from_args();

    std::process::exit({
        match args.execute() {
            Ok(()) => 0,
            Err(err) => {
                eprintln!("Error: {}", err);

                let mut caused_by = err.source();
                while let Some(err) = caused_by {
                    eprintln!("Caused by: {}", err);
                    caused_by = err.source();
                }
                1
            }
        }
    });
}
