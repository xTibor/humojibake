use std::io::{self, BufReader, BufWriter, Read, Write};

use structopt::StructOpt;

use crate::encodings;

use crate::subcommands::Subcommand;

#[derive(StructOpt)]
pub struct ConvertBetweenArgs {
    #[structopt(help = "Source encoding name. See the `supported-encodings` subcommand for supported encodings.")]
    pub source_encoding_name: String,

    #[structopt(help = "Target encoding name. See the `supported-encodings` subcommand for supported encodings.")]
    pub target_encoding_name: String,
}

impl Subcommand for ConvertBetweenArgs {
    fn execute(&self) -> Result<(), crate::error::Error> {
        let source_encoding_table = encodings::from_str(&self.source_encoding_name)?;
        let target_encoding_table = encodings::from_str(&self.target_encoding_name)?;

        let reader = BufReader::new(io::stdin());
        let mut writer = BufWriter::new(io::stdout());

        for b in reader.bytes().filter_map(Result::ok) {
            let c = encodings::decode(source_encoding_table, b);
            let b = encodings::encode(target_encoding_table, c);
            writer.write_all(&[b])?;
        }

        Ok(())
    }
}
