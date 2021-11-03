use structopt::StructOpt;
use strum::{IntoEnumIterator, VariantNames};

use crate::encodings::{self, max_encoding_name_width, ENCODINGS};
use crate::language::Language;
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

    #[structopt(long, help = "Show incompatible language-encoding pairs")]
    pub show_incompatible: bool,

    #[structopt(long, help = "Guess scoring strategy", default_value = "advanced")]
    pub score_strategy: ScoreStrategy,

/* TODO: Turn this into a filter
    #[structopt(
        long,
        help = "Language the guess is based on. See the `list-languages` subcommand for supported languages.",
        default_value = "hungarian"
    )]
    pub language: Language,
*/
}

impl Subcommand for GuessHexArgs {
    fn execute(&self) -> Result<(), crate::error::Error> {
        let bin_string = hexstr_to_vec(&self.input)?;

        let mut results: Vec<(Language, &str, isize, String)> = Vec::new();

        for language in Language::iter() {
            for (encoding_name, encoding_table, encoding_is_common) in ENCODINGS {
                if !encodings::supports_charset(encoding_table, language.get_charset()) && !self.show_incompatible {
                    continue;
                }

                if self.hide_uncommon && !encoding_is_common {
                    continue;
                }

                let decoded = bin_string
                    .iter()
                    .map(|&b| encodings::decode(encoding_table, b))
                    .collect::<Vec<char>>();

                let score = self.score_strategy.eval(language.get_charset(), &decoded);
                let preview_string = decoded.iter().collect();

                results.push((language, encoding_name, score, preview_string));
            }
        }

        results.sort_by_key(|(_, _, score, _)| -score);

        let max_score_width = (results.iter().map(|(_, _, score, _)| *score).max().unwrap_or(0) as f64)
            .log10()
            .ceil() as usize;

        let max_language_width = Language::VARIANTS.iter().map(|s| s.len()).max().unwrap_or(0);

        for (language, encoding_name, score, preview_string) in results {
            print!(
                "{:score_width$} {:language_width$} {:encoding_name_width$}",
                score,
                language,
                encoding_name,
                score_width = max_score_width,
                language_width = max_language_width,
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
