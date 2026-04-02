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
    pub active_animation: Option<AttemptAnimation>,
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

#[derive(Debug, Clone, Copy)]
pub struct AttemptAnimation {
    pub guess: [u8; 5],
    pub progress: [Progress; 5],
    pub target_row: usize,
    pub phase: AttemptAnimationPhase,
    pub phase_elapsed: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttemptAnimationPhase {
    Translate,
    Flip,
    Settle,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Progress {
    #[default]
    Empty,
    Absent,
    Present,
    Correct,
}
