use structopt::StructOpt;

use crate::encodings::{self, max_encoding_name_width, ENCODINGS};
use crate::subcommands::Subcommand;

#[derive(StructOpt)]
pub struct ShowcaseArgs {
    #[structopt(
        help = "Input text",
        default_value = "ÖT HŰTŐHÁZBÓL KÉRTÜNK SZÍNHÚST öt hűtőházból kértünk színhúst"
    )]
    pub input: String,

    #[structopt(long, help = "Hide uncommon encodings")]
    pub hide_uncommon: bool,
}

impl Subcommand for ShowcaseArgs {
    fn execute(&self) -> Result<(), crate::error::Error> {
        for (src_name, src_table, src_is_common) in ENCODINGS {
            if self.hide_uncommon && !src_is_common {
                continue;
            }

            for (dst_name, dst_table, dst_is_common) in ENCODINGS {
                if self.hide_uncommon && !dst_is_common {
                    continue;
                }

                if src_table == dst_table {
                    continue;
                }

                let result = self
                    .input
                    .chars()
                    .map(|c| encodings::encode(src_table, c))
                    .map(|b| encodings::decode(dst_table, b))
                    .collect::<String>();

                println!(
                    "{:encoding_name_width$} {:encoding_name_width$} \"{}\"",
                    src_name,
                    dst_name,
                    result,
                    encoding_name_width = max_encoding_name_width()
                );
            }
        }

        Ok(())
    }
}
