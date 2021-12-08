mod convert_encoding;
mod encode_hex;
mod guess_hex;
mod histogram;
mod showcase;
mod supported_encodings;
mod supported_languages;

use self::convert_encoding::ConvertEncodingArgs;
use self::encode_hex::EncodeHexArgs;
use self::guess_hex::GuessHexArgs;
use self::histogram::HistogramArgs;
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
    ConvertEncoding(ConvertEncodingArgs),
    EncodeHex(EncodeHexArgs),
    GuessHex(GuessHexArgs),
    Histogram(HistogramArgs),
    Showcase(ShowcaseArgs),
    SupportedEncodings(SupportedEncodingsArgs),
    SupportedLanguages(SupportedLanguagesArgs),
}

impl Subcommand for SubcommandArgs {
    fn execute(&self) -> Result<(), Error> {
        match self {
            SubcommandArgs::ConvertEncoding(args) => args.execute(),
            SubcommandArgs::EncodeHex(args) => args.execute(),
            SubcommandArgs::GuessHex(args) => args.execute(),
            SubcommandArgs::Histogram(args) => args.execute(),
            SubcommandArgs::Showcase(args) => args.execute(),
            SubcommandArgs::SupportedEncodings(args) => args.execute(),
            SubcommandArgs::SupportedLanguages(args) => args.execute(),
        }
    }
}
