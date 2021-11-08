use std::io::{self, BufReader, BufWriter, Read, Write};
use std::str::FromStr;

use structopt::StructOpt;

use crate::encodings::Encoding;
use crate::error::Error;
use crate::subcommands::Subcommand;

#[derive(StructOpt)]
pub struct ConvertBetweenArgs {
    #[structopt(help = "Source encoding name. See the `supported-encodings` subcommand for supported encodings.")]
    pub source_encoding_name: String,

    #[structopt(help = "Target encoding name. See the `supported-encodings` subcommand for supported encodings.")]
    pub target_encoding_name: String,
}

impl Subcommand for ConvertBetweenArgs {
    fn execute(&self) -> Result<(), Error> {
        let source_encoding =
            Encoding::from_str(&self.source_encoding_name).map_err(|_| Error::UnsupportedEncoding {
                encoding_name: self.source_encoding_name.clone(),
            })?;

        let target_encoding =
            Encoding::from_str(&self.target_encoding_name).map_err(|_| Error::UnsupportedEncoding {
                encoding_name: self.target_encoding_name.clone(),
            })?;

        let reader = BufReader::new(io::stdin());
        let mut writer = BufWriter::new(io::stdout());

        for b in reader
            .bytes()
            .filter_map(Result::ok)
            .map(|b| source_encoding.decode(b))
            .map(|c| target_encoding.encode(c))
        {
            writer.write_all(&[b])?;
        }

        Ok(())
    }
}
