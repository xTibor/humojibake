use structopt::StructOpt;
use strum::IntoEnumIterator;

use crate::encodings::Encoding;
use crate::error::Error;
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
    fn execute(&self) -> Result<(), Error> {
        Encoding::iter()
            .filter(|encoding| {
                self.languages
                    .iter()
                    .all(|language| encoding.supports_charset(language.get_charset()))
            })
            .for_each(|encoding| println!("{}", encoding));

        Ok(())
    }
}
