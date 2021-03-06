use structopt::StructOpt;
use strum::IntoEnumIterator;

use crate::encodings::{Decoder, Encoder, Encoding};
use crate::error::Error;
use crate::subcommands::Subcommand;

#[derive(StructOpt)]
pub struct ShowcaseArgs {
    #[structopt(
        help = "Input text",
        default_value = "ÖT HŰTŐHÁZBÓL KÉRTÜNK SZÍNHÚST öt hűtőházból kértünk színhúst"
    )]
    pub input: String,

    #[structopt(long, help = "Show uncommon encodings")]
    pub show_uncommon: bool,
}

impl Subcommand for ShowcaseArgs {
    fn execute(&self) -> Result<(), Error> {
        for source_encoding in Encoding::iter() {
            if !self.show_uncommon && !source_encoding.is_common() {
                continue;
            }

            for target_encoding in Encoding::iter() {
                if !self.show_uncommon && !target_encoding.is_common() {
                    continue;
                }

                if source_encoding == target_encoding {
                    continue;
                }

                let result = self
                    .input
                    .chars()
                    .encode(source_encoding)
                    .decode(target_encoding)
                    .collect::<String>();

                println!(
                    "{:encoding_name_width$} {:encoding_name_width$} \"{}\"",
                    source_encoding,
                    target_encoding,
                    result,
                    encoding_name_width = Encoding::max_encoding_name_width()
                );
            }
        }

        Ok(())
    }
}
