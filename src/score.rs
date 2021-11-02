use strum::{EnumString, EnumVariantNames};

#[derive(EnumString, EnumVariantNames)]
#[strum(serialize_all = "kebab_case")]
pub enum ScoreStrategy {
    Simple,
    Advanced,
    // TODO: Explore trigram analysis
}

impl ScoreStrategy {
    pub fn eval(&self, charset: &[char], input: &[char]) -> isize {
        match self {
            ScoreStrategy::Simple => score_strategy_simple(charset, input),
            ScoreStrategy::Advanced => score_strategy_advanced(charset, input),
        }
    }
}

fn score_strategy_simple(charset: &[char], input: &[char]) -> isize {
    input.iter().map(|c| if charset.contains(c) { 1 } else { 0 }).sum()
}

fn score_strategy_advanced(charset: &[char], input: &[char]) -> isize {
    const WORD_SEPARATORS: &[char] = &[
        ' ', '\n', '\t', '\r', '.', '…', '!', '?', ',', '‚', ':', ';', '\'', '"', '’', '„', '”', '«', '»', '‹', '›',
        '(', ')', '[', ']', '-', '–', '—', '+', '/',
    ];

    input
        // Split to words
        .split(|&c| WORD_SEPARATORS.contains(&c))
        // Filter out invalid looking words
        .filter(|&word| word.iter().all(|c| charset.contains(c)))
        // Filter out short words
        .filter(|&word| word.len() >= 4)
        // Filter out words with weird capitalizations (AAAaAAA, aaaAaaa, etc.)
        .filter(|&word| {
            let is_lowercase = word.iter().cloned().all(char::is_lowercase);
            let is_uppercase = word.iter().cloned().all(char::is_uppercase);
            let is_capitalcase = (word[0].is_uppercase()) && (word[1..].iter().cloned().all(char::is_lowercase));

            is_lowercase || is_uppercase || is_capitalcase
        })
        // Cumulative length of the remaining words
        .map(|word| word.len() as isize)
        .sum()
}
