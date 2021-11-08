use std::io::{self, BufReader, BufWriter, Write};

use structopt::StructOpt;
use utf8_chars::BufReadCharsExt;

use crate::encodings::{Encoder, Encoding};
use crate::error::Error;
use crate::subcommands::Subcommand;

#[derive(StructOpt)]
pub struct ConvertFromUtf8Args {
    #[structopt(help = "Target encoding name. See the `supported-encodings` subcommand for supported encodings.")]
    pub target_encoding: Encoding,
}

impl Subcommand for ConvertFromUtf8Args {
    fn execute(&self) -> Result<(), Error> {
        let mut reader = BufReader::new(io::stdin());
        let mut writer = BufWriter::new(io::stdout());

        for b in reader.chars().filter_map(Result::ok).encode(self.target_encoding) {
            writer.write_all(&[b])?;
        }

        Ok(())
    }
}
