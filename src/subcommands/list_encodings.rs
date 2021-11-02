use structopt::StructOpt;

use crate::encodings::ENCODINGS;
use crate::subcommands::Subcommand;

#[derive(StructOpt)]
pub struct ListEncodingsArgs {}

impl Subcommand for ListEncodingsArgs {
    fn execute(&self) -> Result<(), crate::error::Error> {
        for (encoding_name, _, _) in ENCODINGS {
            println!("{}", encoding_name);
        }

        Ok(())
    }
}
