use structopt::StructOpt;
use strum::VariantNames;

use crate::language::Language;
use crate::subcommands::Subcommand;

#[derive(StructOpt)]
pub struct ListLanguagesArgs {}

impl Subcommand for ListLanguagesArgs {
    fn execute(&self) -> Result<(), crate::error::Error> {
        for language_name in Language::VARIANTS {
            println!("{}", language_name);
        }

        Ok(())
    }
}
