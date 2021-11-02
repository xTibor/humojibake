use strum::{EnumString, EnumVariantNames};

#[derive(EnumString, EnumVariantNames)]
#[strum(serialize_all = "kebab_case")]
pub enum ScoreStrategy {
    Simple,
    Advanced,
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
        .filter(|i| i.iter().all(|c| charset.contains(c)))
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
        .sum()
}
