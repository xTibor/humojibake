// TODO: Error handling, less allocation
pub fn hexstr_to_vec(input: &str) -> Vec<u8> {
    input
        .chars()
        .filter(|c| c.is_ascii_hexdigit())
        .collect::<Vec<char>>()
        .chunks_exact(2)
        .map(|chunk| chunk.iter().collect::<String>())
        .map(|s| u8::from_str_radix(&s, 16))
        .filter_map(Result::ok)
        .collect::<Vec<u8>>()
}
