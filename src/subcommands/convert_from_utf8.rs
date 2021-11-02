use std::io::{self, BufReader, BufWriter};

use structopt::StructOpt;

use crate::encodings;
use crate::subcommands::Subcommand;

#[derive(StructOpt)]
pub struct ConvertFromUtf8Args {
    #[structopt(help = "Target encoding name. See the `list-encodings` subcommand for supported encodings.")]
    pub target_encoding_name: String,
}

impl Subcommand for ConvertFromUtf8Args {
    fn execute(&self) -> Result<(), crate::error::Error> {
        let target_encoding_table = encodings::from_str(&self.target_encoding_name)?;

        let reader = BufReader::new(io::stdin());
        let mut writer = BufWriter::new(io::stdout());

        // TODO: Why BufReader has no .chars() anymore?
        todo!();
    }
}
