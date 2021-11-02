pub mod convert_between;
pub mod convert_from_utf8;
pub mod convert_to_utf8;
pub mod encode_hex;
pub mod guess_hex;
pub mod list;
pub mod showcase;

use self::convert_between::ConvertBetweenArgs;
use self::convert_from_utf8::ConvertFromUtf8Args;
use self::convert_to_utf8::ConvertToUtf8Args;
use self::encode_hex::EncodeHexArgs;
use self::guess_hex::GuessHexArgs;
use self::list::ListArgs;
use self::showcase::ShowcaseArgs;

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
    List(ListArgs),
    Showcase(ShowcaseArgs),
}

impl Subcommand for SubcommandArgs {
    fn execute(&self) -> Result<(), Error> {
        match self {
            SubcommandArgs::ConvertBetween(args) => args.execute(),
            SubcommandArgs::ConvertFromUtf8(args) => args.execute(),
            SubcommandArgs::ConvertToUtf8(args) => args.execute(),
            SubcommandArgs::EncodeHex(args) => args.execute(),
            SubcommandArgs::GuessHex(args) => args.execute(),
            SubcommandArgs::List(args) => args.execute(),
            SubcommandArgs::Showcase(args) => args.execute(),
        }
    }
}
