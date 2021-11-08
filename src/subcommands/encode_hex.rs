use structopt::StructOpt;

use crate::encodings::{Encoder, Encoding};
use crate::error::Error;
use crate::subcommands::Subcommand;

#[derive(StructOpt)]
pub struct EncodeHexArgs {
    #[structopt(help = "Target encoding name. See the `list` subcommand for supported encodings.")]
    pub target_encoding: Encoding,

    #[structopt(help = "Input text")]
    pub input: String,
}

impl Subcommand for EncodeHexArgs {
    fn execute(&self) -> Result<(), Error> {
        let result = self
            .input
            .chars()
            .encode(self.target_encoding)
            .map(|b| format!("{:02X}", b))
            .intersperse(" ".to_owned())
            .collect::<String>();

        println!("{}", result);

        Ok(())
    }
}
