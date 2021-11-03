use std::io::{self, BufReader, BufWriter};
use std::str::FromStr;

use structopt::StructOpt;

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

        let reader = BufReader::new(io::stdin());
        let mut writer = BufWriter::new(io::stdout());

        // TODO: Why BufReader has no .chars() anymore?
        todo!();
    }
}
