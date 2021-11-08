use std::io::{self, BufReader, BufWriter, Read, Write};

use structopt::StructOpt;

use crate::encodings::{Decoder, Encoder, Encoding};
use crate::error::Error;
use crate::subcommands::Subcommand;

#[derive(StructOpt)]
pub struct ConvertBetweenArgs {
    #[structopt(help = "Source encoding name. See the `supported-encodings` subcommand for supported encodings.")]
    pub source_encoding: Encoding,

    #[structopt(help = "Target encoding name. See the `supported-encodings` subcommand for supported encodings.")]
    pub target_encoding: Encoding,
}

impl Subcommand for ConvertBetweenArgs {
    fn execute(&self) -> Result<(), Error> {
        let reader = BufReader::new(io::stdin());
        let mut writer = BufWriter::new(io::stdout());

        for b in reader
            .bytes()
            .filter_map(Result::ok)
            .decode(self.source_encoding)
            .encode(self.target_encoding)
        {
            writer.write_all(&[b])?;
        }

        Ok(())
    }
}
