use std::str::FromStr;

use structopt::StructOpt;

use crate::encodings::Encoding;
use crate::error::Error;
use crate::subcommands::Subcommand;

#[derive(StructOpt)]
pub struct EncodeHexArgs {
    #[structopt(help = "Target encoding name. See the `list` subcommand for supported encodings.")]
    pub target_encoding_name: String,

    #[structopt(help = "Input text")]
    pub input: String,
}

impl Subcommand for EncodeHexArgs {
    fn execute(&self) -> Result<(), crate::error::Error> {
        let target_encoding =
            Encoding::from_str(&self.target_encoding_name).map_err(|_| Error::UnsupportedEncoding {
                encoding_name: self.target_encoding_name.clone(),
            })?;

        let result = self
            .input
            .chars()
            .map(|c| target_encoding.encode(c))
            .map(|b| format!("{:02X}", b))
            .intersperse(" ".to_owned())
            .collect::<String>();

        println!("{}", result);

        Ok(())
    }
}
