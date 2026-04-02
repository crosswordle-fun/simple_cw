use crate::config::WORD_LIST;

#[derive(Debug, Default)]
pub struct WordleInput {
    pub input: [u8; 5],
    pub cursor: u8,
}

#[derive(Debug, Default)]
pub struct WordleGame {
    pub curr_level: WordleLevel,
    pub prev_levels: Vec<WordleLevel>,
}

#[derive(Debug, Default)]
pub struct WordleLevel {
    pub solution: [u8; 5],
    pub attempts: [Attempt; 5],
}

pub fn new_wordle_level(word_idx: usize) -> WordleLevel {
    let solution = WORD_LIST[word_idx]
        .to_ascii_uppercase()
        .as_bytes()
        .try_into()
        .unwrap();

    WordleLevel {
        solution,
        ..Default::default()
    }
}

#[derive(Debug, Default)]
pub struct Attempt {
    pub word: [u8; 5],
    pub progress: [Progress; 5],
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Progress {
    #[default]
    Empty,
    Absent,
    Present,
    Correct,
}
