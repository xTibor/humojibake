use std::io::{self, BufReader, Read};

use iterslide::SlideIterator;
use itertools::Itertools;
use structopt::StructOpt;

use crate::encodings::{Encoder, Encoding};
use crate::error::Error;
use crate::subcommands::Subcommand;

#[derive(StructOpt)]
pub struct SearchDeltaspaceArgs {
    #[structopt(help = "Input text")]
    pub input: String,

    #[structopt(
        long,
        help = "Character encoding used for the delta transform. See the `supported-encodings` subcommand for supported encodings."
    )]
    pub encoding: Encoding,
    // TODO: U16LE, U16BE, U32LE, U32BE
}

impl Subcommand for SearchDeltaspaceArgs {
    fn execute(&self) -> Result<(), Error> {
        let input_delta = self
            .input
            .chars()
            .encode(self.encoding)
            .tuple_windows()
            .map(|(a, b)| b as isize - a as isize)
            .collect::<Vec<isize>>();

        BufReader::new(io::stdin())
            .bytes()
            .filter_map(Result::ok)
            .tuple_windows()
            .map(|(a, b)| b as isize - a as isize)
            .slide(input_delta.len())
            .enumerate()
            .filter(|(_, window_delta)| window_delta == &input_delta)
            .for_each(|(position, _)| println!("{:08X} U8", position));

        Ok(())
    }
}
