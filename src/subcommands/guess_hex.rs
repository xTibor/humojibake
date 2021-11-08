use structopt::StructOpt;
use strum::IntoEnumIterator;

use crate::encodings::{Decoder, Encoding};
use crate::error::Error;
use crate::language::Language;
use crate::score::ScoreStrategy;
use crate::subcommands::Subcommand;
use crate::utils::{hexstr_to_vec, DisplayWidth};

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

    #[structopt(
        long,
        help = "Filter guesses to these languages. See the `list-languages` subcommand for supported languages."
    )]
    pub filter_language: Option<Vec<Language>>,

    #[structopt(
        long,
        help = "Filter guesses to these encodings. See the `list-encodings` subcommand for supported encodings."
    )]
    pub filter_encoding: Option<Vec<Encoding>>,
}

impl Subcommand for GuessHexArgs {
    fn execute(&self) -> Result<(), Error> {
        let bin_string = hexstr_to_vec(&self.input)?;

        let mut results: Vec<(Language, Encoding, usize, String)> = Vec::new();

        for language in Language::iter() {
            if let Some(filter_language) = &self.filter_language {
                if !filter_language.contains(&language) {
                    continue;
                }
            }

            for encoding in Encoding::iter() {
                if let Some(filter_encoding) = &self.filter_encoding {
                    if !filter_encoding.contains(&encoding) {
                        continue;
                    }
                }

                if !encoding.supports_charset(language.get_charset()) && !self.show_incompatible {
                    continue;
                }

                if self.hide_uncommon && !encoding.is_common() {
                    continue;
                }

                let decoded = bin_string.iter().decode(encoding).collect::<Vec<char>>();

                let score = self.score_strategy.eval(language.get_charset(), &decoded);
                let preview_string = decoded.iter().collect();

                results.push((language, encoding, score, preview_string));
            }
        }

        results.sort_by_key(|&(_, _, score, _)| score);
        results.reverse();

        let max_score_width = results.iter().map(|(_, _, score, _)| *score).display_width();

        for (language, encoding_name, score, preview_string) in results {
            print!(
                "{:score_width$} {:language_width$} {:encoding_name_width$}",
                score,
                language,
                encoding_name,
                score_width = max_score_width,
                language_width = Language::max_language_name_width(),
                encoding_name_width = Encoding::max_encoding_name_width(),
            );

            if !self.hide_preview {
                print!(" \"{}\"", preview_string);
            }

            println!();
        }

        Ok(())
    }
}
