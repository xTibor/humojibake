mod convert_between;
mod convert_from_utf8;
mod convert_to_utf8;
mod encode_hex;
mod guess_hex;
mod showcase;
mod supported_encodings;
mod supported_languages;

use self::convert_between::ConvertBetweenArgs;
use self::convert_from_utf8::ConvertFromUtf8Args;
use self::convert_to_utf8::ConvertToUtf8Args;
use self::encode_hex::EncodeHexArgs;
use self::guess_hex::GuessHexArgs;
use self::showcase::ShowcaseArgs;
use self::supported_encodings::SupportedEncodingsArgs;
use self::supported_languages::SupportedLanguagesArgs;

use structopt::StructOpt;

use crate::error::Error;

pub trait Subcommand {
    fn execute(&self) -> Result<(), Error>;
}

#[derive(StructOpt)]
pub enum SubcommandArgs {
    ConvertBetween(ConvertBetweenArgs),
    ConvertFromUtf8(ConvertFromUtf8Args),
    ConvertToUtf8(ConvertToUtf8Args),
    EncodeHex(EncodeHexArgs),
    GuessHex(GuessHexArgs),
    Showcase(ShowcaseArgs),
    SupportedEncodings(SupportedEncodingsArgs),
    SupportedLanguages(SupportedLanguagesArgs),
}

impl Subcommand for SubcommandArgs {
    fn execute(&self) -> Result<(), Error> {
        match self {
            SubcommandArgs::ConvertBetween(args) => args.execute(),
            SubcommandArgs::ConvertFromUtf8(args) => args.execute(),
            SubcommandArgs::ConvertToUtf8(args) => args.execute(),
            SubcommandArgs::EncodeHex(args) => args.execute(),
            SubcommandArgs::GuessHex(args) => args.execute(),
            SubcommandArgs::Showcase(args) => args.execute(),
            SubcommandArgs::SupportedEncodings(args) => args.execute(),
            SubcommandArgs::SupportedLanguages(args) => args.execute(),
        }
    }
}
