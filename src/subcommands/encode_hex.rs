use structopt::StructOpt;

use crate::encodings;
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
        let encoding_table = encodings::from_str(&self.target_encoding_name)?;

        let result = self
            .input
            .chars()
            .map(|c| encodings::encode(encoding_table, c))
            .map(|b| format!("{:02X}", b))
            .intersperse(" ".to_owned())
            .collect::<String>();

        println!("{}", result);

        Ok(())
    }
}
