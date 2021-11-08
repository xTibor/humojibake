use std::io::{self, BufReader, BufWriter, Read, Write};

use structopt::StructOpt;

use crate::encodings::{Decoder, Encoding};
use crate::error::Error;
use crate::subcommands::Subcommand;

#[derive(StructOpt)]
pub struct ConvertToUtf8Args {
    #[structopt(help = "Source encoding name. See the `supported-encodings` subcommand for supported encodings.")]
    pub source_encoding: Encoding,
}

impl Subcommand for ConvertToUtf8Args {
    fn execute(&self) -> Result<(), Error> {
        let reader = BufReader::new(io::stdin());
        let mut writer = BufWriter::new(io::stdout());

        for c in reader.bytes().filter_map(Result::ok).decode(self.source_encoding) {
            write!(writer, "{}", c)?;
        }

        Ok(())
    }
}
