#![allow(unused)]
#![feature(iter_intersperse)]

mod encodings;
use encodings::*;

mod utils;
use utils::hexstr_to_vec;

mod error;
use error::Error;

use structopt::StructOpt;

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
        // TODO: Switch between simple and advanced scoring
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
    StreamToUtf8 {
        #[structopt(help = "Source encoding name. See the `list` subcommand for supported encodings.")]
        source_encoding_name: String,
    },
    StreamFromUtf8 {
        #[structopt(help = "Target encoding name. See the `list` subcommand for supported encodings.")]
        target_encoding_name: String,
    },
    StreamBetween {
        #[structopt(help = "Source encoding name. See the `list` subcommand for supported encodings.")]
        source_encoding_name: String,
        #[structopt(help = "Target encoding name. See the `list` subcommand for supported encodings.")]
        target_encoding_name: String,
    },
}

#[rustfmt::skip]
const HUNGARIAN_CHARSET: &[char] = &[
    'A', 'Á', 'B', 'C', 'D', 'E', 'É', 'F', 'G', 'H', 'I', 'Í', 'J', 'K', 'L', 'M',
    'N', 'O', 'Ó', 'Ö', 'Ő', 'P', 'Q', 'R', 'S', 'T', 'U', 'Ú', 'Ü', 'Ű', 'V', 'W',
    'X', 'Y', 'Z',
    'a', 'á', 'b', 'c', 'd', 'e', 'é', 'f', 'g', 'h', 'i', 'í', 'j', 'k', 'l', 'm',
    'n', 'o', 'ó', 'ö', 'ő', 'p', 'q', 'r', 's', 't', 'u', 'ú', 'ü', 'ű', 'v', 'w',
    'x', 'y', 'z',
];

#[rustfmt::skip]
const HUNGARIAN_ACCENTED_CHARSET: &[char] = &[
    'Á', 'É', 'Í', 'Ú', 'Ü', 'Ű', 'Ó', 'Ö', 'Ő',
    'á', 'é', 'í', 'ú', 'ü', 'ű', 'ó', 'ö', 'ő',
];

const WORD_SEPARATORS: &[char] = &[
    ' ', '\n', '\t', '\r', '.', '…', '!', '?', ',', '‚', ':', ';', '\'', '"', '’', '„', '”', '«', '»', '‹', '›', '(',
    ')', '[', ']', '-', '–', '—', '+', '/',
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
                    .map(|&b| encoding_table[b as usize])
                    .collect::<Vec<char>>();

                let score_simple: isize = decoded
                    .iter()
                    .map(|c| if HUNGARIAN_CHARSET.contains(c) { 1 } else { 0 })
                    .sum();

                let score_advanced: isize = decoded
                    // Split to words
                    .split(|&c| WORD_SEPARATORS.contains(&c))
                    // Filter out invalid looking words
                    .filter(|i| i.iter().all(|c| HUNGARIAN_CHARSET.contains(c)))
                    // Convert char array to string
                    .map(|i| i.iter().collect::<String>())
                    // Filter out short words
                    .filter(|s| s.chars().count() >= 4)
                    // Filter out words with weird capitalizations (AAAaAAA, aaaAaaa, etc.)
                    .filter(|s| {
                        let is_lowercase = *s == s.to_lowercase();
                        let is_uppercase = *s == s.to_uppercase();
                        let is_capitalcase = {
                            // This unwrap() cannot fail because of the length filtering above
                            let (left, right) = (s.chars().next().unwrap(), s.chars().skip(1).collect::<String>());
                            (left.is_uppercase()) && (right == right.to_lowercase())
                        };
                        is_lowercase || is_uppercase || is_capitalcase
                    })
                    // Cumulative length of the remaining words
                    .map(|s| s.chars().count() as isize)
                    .sum();

                let preview_string = bin_string.iter().map(|&b| encoding_table[b as usize]).collect();

                results.push((encoding_name, score_advanced, preview_string));
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
                .map(|c| encoding_table.iter().position(|&p| p == c).unwrap_or(0) as u8)
                .map(|b| format!("{:02X}", b))
                .intersperse(" ".to_owned())
                .collect::<String>();

            println!("{}", result);
        }
        AppArgs::Showcase { input, hide_uncommon } => {
            for (src_name, &src_table, src_is_common) in ENCODINGS {
                if hide_uncommon && !src_is_common {
                    continue;
                }

                for (dst_name, &dst_table, dst_is_common) in ENCODINGS {
                    if hide_uncommon && !dst_is_common {
                        continue;
                    }

                    if src_table == dst_table {
                        continue;
                    }

                    let result = input
                        .chars()
                        .map(|c| dst_table[src_table.iter().position(|&p| p == c).unwrap_or(0)])
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
        AppArgs::StreamToUtf8 { source_encoding_name } => {
            let source_encoding_table = encodings::from_str(&source_encoding_name)?;

            todo!();
        }
        AppArgs::StreamFromUtf8 { target_encoding_name } => {
            let target_encoding_table = encodings::from_str(&target_encoding_name)?;

            todo!();
        }
        AppArgs::StreamBetween {
            source_encoding_name,
            target_encoding_name,
        } => {
            let source_encoding_table = encodings::from_str(&source_encoding_name)?;
            let target_encoding_table = encodings::from_str(&target_encoding_name)?;

            todo!();
        }
    }

    Ok(())
}
