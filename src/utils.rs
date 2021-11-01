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
