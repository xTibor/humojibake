use std::io::{self, BufReader, BufWriter, Read, Write};

use structopt::StructOpt;
use utf8_chars::BufReadCharsExt;

use crate::encodings::{Decoder, Encoder, Encoding};
use crate::error::Error;
use crate::subcommands::Subcommand;

#[derive(StructOpt)]
pub struct ConvertEncodingArgs {
    #[structopt(
        long,
        help = "Source encoding name. See the `supported-encodings` subcommand for supported encodings."
    )]
    pub source_encoding: Option<Encoding>,

    #[structopt(
        long,
        help = "Target encoding name. See the `supported-encodings` subcommand for supported encodings."
    )]
    pub target_encoding: Option<Encoding>,
}

impl Subcommand for ConvertEncodingArgs {
    fn execute(&self) -> Result<(), Error> {
        let mut reader = BufReader::new(io::stdin());
        let mut writer = BufWriter::new(io::stdout());

        match (self.source_encoding, self.target_encoding) {
            (Some(source_encoding), Some(target_encoding)) => {
                reader
                    .bytes()
                    .filter_map(Result::ok)
                    .decode(source_encoding)
                    .encode(target_encoding)
                    .try_for_each(|b| writer.write_all(&[b]))?;
            }

            (Some(source_encoding), None) => {
                reader
                    .bytes()
                    .filter_map(Result::ok)
                    .decode(source_encoding)
                    .try_for_each(|c| write!(writer, "{}", c))?;
            }

            (None, Some(target_encoding)) => {
                reader
                    .chars()
                    .filter_map(Result::ok)
                    .encode(target_encoding)
                    .try_for_each(|b| writer.write_all(&[b]))?;
            }

            (None, None) => {
                io::copy(&mut reader, &mut writer)?;
            }
        }

        Ok(())
    }
}
