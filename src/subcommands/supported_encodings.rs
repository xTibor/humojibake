use structopt::StructOpt;

use crate::encodings::{self, ENCODINGS};
use crate::language::Language;
use crate::subcommands::Subcommand;

#[derive(StructOpt)]
pub struct SupportedEncodingsArgs {
    #[structopt(
        help = "List of languages that encodings must support. See the `supported-languages` subcommand for supported languages."
    )]
    pub languages: Vec<Language>,
}

impl Subcommand for SupportedEncodingsArgs {
    fn execute(&self) -> Result<(), crate::error::Error> {
        ENCODINGS
            .iter()
            .filter(|(_, encoding_table, _)| {
                self.languages
                    .iter()
                    .all(|lang| encodings::supports_charset(encoding_table, lang.get_charset()))
            })
            .for_each(|(encoding_name, _, _)| println!("{}", encoding_name));

        Ok(())
    }
}
