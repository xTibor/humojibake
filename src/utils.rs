use crate::error::Error;

// TODO: Error handling, less allocation
pub fn hexstr_to_vec(input: &str) -> Result<Vec<u8>, Error> {
    Ok(input
        .chars()
        .filter(|c| !c.is_ascii_whitespace())
        .collect::<Vec<char>>()
        .chunks_exact(2)
        .map(|chunk| chunk.iter().collect::<String>())
        .map(|s| u8::from_str_radix(&s, 16))
        .filter_map(Result::ok)
        .collect::<Vec<u8>>())
}

pub trait DisplayWidth {
    fn display_width(self) -> usize;
}

impl<T> DisplayWidth for T
where
    T: Iterator<Item = usize>,
{
    fn display_width(self) -> usize {
        (self.max().unwrap_or(0) as f64).log10().ceil() as usize
    }
}
