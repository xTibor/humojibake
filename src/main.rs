#![allow(unused)]
#![feature(iter_intersperse)]

mod encodings;
use std::fmt::format;

use encodings::*;

mod utils;
use utils::hexstr_to_vec;

use structopt::StructOpt;

#[derive(StructOpt)]
enum AppArgs {
    List,
    Guess {
        #[structopt(help = "Hexadecimal input")]
        input: String,
        #[structopt(long, help = "Hide the preview text")]
        hide_preview: bool,
        #[structopt(long, help = "Hide uncommon encodings")]
        hide_uncommon: bool,
        #[structopt(long, help = "Show encodings with partial or no Hungarian language support")]
        show_foreign: bool,
    },
    Encode {
        #[structopt(help = "Encoding name. See the `list` subcommand for supported encodings.")]
        encoding_name: String,
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

fn main() {
    let args = AppArgs::from_args();

    match args {
        AppArgs::List => {
            for (encoding_name, _, _) in ENCODINGS {
                println!("{}", encoding_name);
            }
        }
        AppArgs::Guess {
            input,
            hide_preview,
            hide_uncommon,
            show_foreign,
        } => {
            let bin_string = hexstr_to_vec(&input);

            let mut results: Vec<(&str, isize, String)> = Vec::new();

            for (encoding_name, encoding_table, encoding_is_common) in ENCODINGS {
                if !encodings::supports_charset(encoding_table, HUNGARIAN_CHARSET) && !show_foreign {
                    continue;
                }

                if hide_uncommon && !encoding_is_common {
                    continue;
                }

                let score = bin_string
                    .iter()
                    .map(|&b| encoding_table[b as usize])
                    .map(|c| if HUNGARIAN_CHARSET.contains(&c) { 1 } else { 0 })
                    .sum();
                // TODO: Penalize weird capitalizations ("aÁaaaaa", "AAAAáAAA")

                let preview_string = bin_string.iter().map(|&b| encoding_table[b as usize]).collect();

                results.push((encoding_name, score, preview_string));
            }

            results.sort_by_key(|entry| -entry.1);

            let max_score_width = (results.iter().map(|result| result.1).max().unwrap_or(0) as f64)
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
        AppArgs::Encode { input, encoding_name } => {
            let encoding_table = encodings::from_str(&encoding_name).unwrap();

            let result = input
                .chars()
                .map(|c| encoding_table.iter().position(|&p| p == c).unwrap() as u8)
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
    }
}
