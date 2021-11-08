use std::collections::HashMap;

use structopt::StructOpt;

use crate::encodings::Encoding;
use crate::error::Error;
use crate::subcommands::Subcommand;
use crate::utils::hexstr_to_vec;

#[derive(StructOpt)]
pub struct HistogramArgs {
    #[structopt(help = "Hexadecimal input")]
    pub input: String,

    #[structopt(help = "Source encoding name. See the `supported-encodings` subcommand for supported encodings.")]
    pub source_encoding: Option<Encoding>,
}

impl Subcommand for HistogramArgs {
    fn execute(&self) -> Result<(), Error> {
        let histogram = {
            let mut result = hexstr_to_vec(&self.input)?
                .iter()
                .fold(HashMap::new(), |mut hashmap, &b| {
                    *hashmap.entry(b).or_insert(0) += 1;
                    hashmap
                })
                .into_iter()
                .collect::<Vec<(u8, i32)>>();
            result.sort();
            result
        };

        // TODO: max_count_width

        histogram.iter().for_each(|&(b, count)| {
            print!("{:02X} {}", b, count);

            if let Some(source_encoding) = &self.source_encoding {
                print!(" \"{}\"", source_encoding.decode(b));
            }

            println!();
        });

        Ok(())
    }
}
