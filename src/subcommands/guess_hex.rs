use structopt::StructOpt;

use crate::encodings::{self, max_encoding_name_width, ENCODINGS};
use crate::score::ScoreStrategy;
use crate::subcommands::Subcommand;
use crate::utils::hexstr_to_vec;

#[derive(StructOpt)]
pub struct GuessHexArgs {
    #[structopt(help = "Hexadecimal input")]
    pub input: String,

    #[structopt(long, help = "Hide the preview text")]
    pub hide_preview: bool,

    #[structopt(long, help = "Hide uncommon encodings")]
    pub hide_uncommon: bool,

    #[structopt(long, help = "Show encodings with partial or no Hungarian language support")]
    pub show_foreign: bool,

    #[structopt(long, help = "Guess scoring strategy", default_value = "advanced")]
    pub score_strategy: ScoreStrategy,
}

#[rustfmt::skip]
pub const HUNGARIAN_CHARSET: &[char] = &[
    'A', 'Á', 'B', 'C', 'D', 'E', 'É', 'F', 'G', 'H', 'I', 'Í', 'J', 'K', 'L', 'M',
    'N', 'O', 'Ó', 'Ö', 'Ő', 'P', 'Q', 'R', 'S', 'T', 'U', 'Ú', 'Ü', 'Ű', 'V', 'W',
    'X', 'Y', 'Z',
    'a', 'á', 'b', 'c', 'd', 'e', 'é', 'f', 'g', 'h', 'i', 'í', 'j', 'k', 'l', 'm',
    'n', 'o', 'ó', 'ö', 'ő', 'p', 'q', 'r', 's', 't', 'u', 'ú', 'ü', 'ű', 'v', 'w',
    'x', 'y', 'z',
];

impl Subcommand for GuessHexArgs {
    fn execute(&self) -> Result<(), crate::error::Error> {
        let bin_string = hexstr_to_vec(&self.input)?;

        let mut results: Vec<(&str, isize, String)> = Vec::new();

        for (encoding_name, encoding_table, encoding_is_common) in ENCODINGS {
            if !encodings::supports_charset(encoding_table, HUNGARIAN_CHARSET) && !self.show_foreign {
                continue;
            }

            if self.hide_uncommon && !encoding_is_common {
                continue;
            }

            let decoded = bin_string
                .iter()
                .map(|&b| encodings::decode(encoding_table, b))
                .collect::<Vec<char>>();

            let score = self.score_strategy.eval(HUNGARIAN_CHARSET, &decoded);
            let preview_string = decoded.iter().collect();

            results.push((encoding_name, score, preview_string));
        }

        results.sort_by_key(|(_, score, _)| -score);

        let max_score_width = (results.iter().map(|(_, score, _)| *score).max().unwrap_or(0) as f64)
            .log10()
            .ceil() as usize;

        for (encoding_name, score, preview_string) in results {
            print!(
                "{:score_width$} {:encoding_name_width$}",
                score,
                encoding_name,
                score_width = max_score_width,
                encoding_name_width = max_encoding_name_width()
            );

            if !self.hide_preview {
                print!(" \"{}\"", preview_string);
            }

            println!();
        }

        Ok(())
    }
}
