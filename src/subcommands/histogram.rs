use std::collections::HashMap;

use structopt::StructOpt;

use crate::encodings::Encoding;
use crate::error::Error;
use crate::subcommands::Subcommand;
use crate::utils::{hexstr_to_vec, DisplayWidth};

#[derive(StructOpt)]
pub struct HistogramArgs {
    #[structopt(help = "Hexadecimal input")]
    pub input: String,

    #[structopt(help = "Source encoding name. See the `supported-encodings` subcommand for supported encodings.")]
    pub source_encoding: Option<Encoding>,

    #[structopt(long, help = "Sorts results by count")]
    pub sort_by_count: bool,
}

impl Subcommand for HistogramArgs {
    fn execute(&self) -> Result<(), Error> {
        let histogram = {
            let mut results = hexstr_to_vec(&self.input)?
                .iter()
                .fold(HashMap::new(), |mut hashmap, &b| {
                    *hashmap.entry(b).or_insert(0) += 1;
                    hashmap
                })
                .into_iter()
                .collect::<Vec<(u8, usize)>>();

            if self.sort_by_count {
                results.sort_by_key(|&(_, count)| count);
                results.reverse();
            } else {
                results.sort_by_key(|&(b, _)| b);
            }

            results
        };

        let max_count_width = histogram.iter().map(|(_, score)| *score).display_width();

        histogram.iter().for_each(|&(b, count)| {
            print!("{:02X} {:count_width$}", b, count, count_width = max_count_width);

            if let Some(source_encoding) = &self.source_encoding {
                print!(" \"{}\"", source_encoding.decode(b));
            }

            println!();
        });

        Ok(())
    }
}
