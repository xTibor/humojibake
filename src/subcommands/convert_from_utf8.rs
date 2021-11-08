use std::io::{self, BufReader, BufWriter, Write};
use std::str::FromStr;

use structopt::StructOpt;
use utf8_chars::BufReadCharsExt;

use crate::encodings::Encoding;
use crate::error::Error;
use crate::subcommands::Subcommand;

#[derive(StructOpt)]
pub struct ConvertFromUtf8Args {
    #[structopt(help = "Target encoding name. See the `supported-encodings` subcommand for supported encodings.")]
    pub target_encoding_name: String,
}

impl Subcommand for ConvertFromUtf8Args {
    fn execute(&self) -> Result<(), crate::error::Error> {
        let target_encoding =
            Encoding::from_str(&self.target_encoding_name).map_err(|_| Error::UnsupportedEncoding {
                encoding_name: self.target_encoding_name.clone(),
            })?;

        let mut reader = BufReader::new(io::stdin());
        let mut writer = BufWriter::new(io::stdout());

        for b in reader.chars().filter_map(Result::ok).map(|c| target_encoding.encode(c)) {
            writer.write_all(&[b])?;
        }

        Ok(())
    }
}
