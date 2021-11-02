#![allow(unused)]
#![feature(iter_intersperse)]

mod encodings;
use encodings::{decode, encode, max_encoding_name_width, ENCODINGS};

mod utils;
use utils::hexstr_to_vec;

mod score;
use score::ScoreStrategy;

mod error;
use error::Error;

use structopt::StructOpt;

use std::io::{self, BufReader, BufWriter, Read, Write};

#[derive(StructOpt)]
enum AppArgs {
    List,
    GuessHex {
        #[structopt(help = "Hexadecimal input")]
        input: String,
        #[structopt(long, help = "Hide the preview text")]
        hide_preview: bool,
        #[structopt(long, help = "Hide uncommon encodings")]
        hide_uncommon: bool,
        #[structopt(long, help = "Show encodings with partial or no Hungarian language support")]
        show_foreign: bool,
        #[structopt(long, help = "Guess scoring strategy", default_value = "advanced")]
        score_strategy: ScoreStrategy,
    },
    EncodeHex {
        #[structopt(help = "Target encoding name. See the `list` subcommand for supported encodings.")]
        target_encoding_name: String,
        #[structopt(help = "Input text")]
        input: String,
    },
    Showcase {
        #[structopt(
            help = "Input text",
            default_value = "ÖT HŰTŐHÁZBÓL KÉRTÜNK SZÍNHÚST öt hűtőházból kértünk színhúst"
        )]
        input: String,
        #[structopt(long, help = "Hide uncommon encodings")]
        hide_uncommon: bool,
    },
    ConvertToUtf8 {
        #[structopt(help = "Source encoding name. See the `list` subcommand for supported encodings.")]
        source_encoding_name: String,
    },
    ConvertFromUtf8 {
        #[structopt(help = "Target encoding name. See the `list` subcommand for supported encodings.")]
        target_encoding_name: String,
    },
    ConvertBetween {
        #[structopt(help = "Source encoding name. See the `list` subcommand for supported encodings.")]
        source_encoding_name: String,
        #[structopt(help = "Target encoding name. See the `list` subcommand for supported encodings.")]
        target_encoding_name: String,
    },
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

fn main() -> Result<(), Error> {
    let args = AppArgs::from_args();

    match args {
        AppArgs::List => {
            for (encoding_name, _, _) in ENCODINGS {
                println!("{}", encoding_name);
            }
        }
        AppArgs::GuessHex {
            input,
            hide_preview,
            hide_uncommon,
            show_foreign,
            score_strategy,
        } => {
            let bin_string = hexstr_to_vec(&input)?;

            let mut results: Vec<(&str, isize, String)> = Vec::new();

            for (encoding_name, encoding_table, encoding_is_common) in ENCODINGS {
                if !encodings::supports_charset(encoding_table, HUNGARIAN_CHARSET) && !show_foreign {
                    continue;
                }

                if hide_uncommon && !encoding_is_common {
                    continue;
                }

                let decoded = bin_string
                    .iter()
                    .map(|&b| encodings::decode(encoding_table, b))
                    .collect::<Vec<char>>();

                let score = score_strategy.eval(HUNGARIAN_CHARSET, &decoded);
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

                if !hide_preview {
                    print!(" \"{}\"", preview_string);
                }

                println!();
            }
        }
        AppArgs::EncodeHex {
            input,
            target_encoding_name,
        } => {
            let encoding_table = encodings::from_str(&target_encoding_name)?;

            let result = input
                .chars()
                .map(|c| encodings::encode(encoding_table, c))
                .map(|b| format!("{:02X}", b))
                .intersperse(" ".to_owned())
                .collect::<String>();

            println!("{}", result);
        }
        AppArgs::Showcase { input, hide_uncommon } => {
            for (src_name, src_table, src_is_common) in ENCODINGS {
                if hide_uncommon && !src_is_common {
                    continue;
                }

                for (dst_name, dst_table, dst_is_common) in ENCODINGS {
                    if hide_uncommon && !dst_is_common {
                        continue;
                    }

                    if src_table == dst_table {
                        continue;
                    }

                    let result = input
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
        }
        AppArgs::ConvertToUtf8 { source_encoding_name } => {
            let source_encoding_table = encodings::from_str(&source_encoding_name)?;

            let reader = BufReader::new(io::stdin());
            let mut writer = BufWriter::new(io::stdout());

            for b in reader.bytes().filter_map(Result::ok) {
                let c = encodings::decode(source_encoding_table, b);
                write!(writer, "{}", c).unwrap();
            }
        }
        AppArgs::ConvertFromUtf8 { target_encoding_name } => {
            let target_encoding_table = encodings::from_str(&target_encoding_name)?;

            let reader = BufReader::new(io::stdin());
            let mut writer = BufWriter::new(io::stdout());

            // TODO: Why BufReader has no .chars() anymore?
            todo!();
        }
        AppArgs::ConvertBetween {
            source_encoding_name,
            target_encoding_name,
        } => {
            let source_encoding_table = encodings::from_str(&source_encoding_name)?;
            let target_encoding_table = encodings::from_str(&target_encoding_name)?;

            let reader = BufReader::new(io::stdin());
            let mut writer = BufWriter::new(io::stdout());

            for b in reader.bytes().filter_map(Result::ok) {
                let c = encodings::decode(source_encoding_table, b);
                let b = encodings::encode(target_encoding_table, c);
                writer.write_all(&[b]).unwrap();
            }
        }
    }

    Ok(())
}
