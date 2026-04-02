use crate::{
    config::{
        WORDLE_FLIP_DURATION, WORDLE_FLIP_STAGGER, WORDLE_INPUT_CLICK_DURATION,
        WORDLE_SETTLE_DURATION, WORDLE_TRANSLATE_DURATION,
    },
    cw_types::{
        Attempt, AttemptAnimation, AttemptAnimationPhase, InputTileAnimation,
        InputTileAnimationKind, WordleGame, WordleInput,
    },
    helpers::wordle_progress,
};
use macroquad::prelude::*;

pub fn handle_wordle_input(wordle_input: &mut WordleInput, wordle_game: &mut WordleGame) {
    if wordle_game.active_animation.is_some() {
        return;
    }

    while let Some(c) = get_char_pressed() {
        if c.is_ascii_alphabetic() {
            let letter = c.to_ascii_uppercase();
            if wordle_input.cursor < 5 {
                let idx = wordle_input.cursor as usize;
                wordle_input.input[idx] = letter as u8;
                wordle_input.tile_animations[idx] = InputTileAnimation {
                    remaining: WORDLE_INPUT_CLICK_DURATION,
                    letter: letter as u8,
                    kind: InputTileAnimationKind::Insert,
                };
                wordle_input.cursor += 1;
            }
        }
    }

    if is_key_pressed(KeyCode::Backspace) {
        if wordle_input.cursor > 0 {
            wordle_input.cursor -= 1;
            let idx = wordle_input.cursor as usize;
            let removed_letter = wordle_input.input[idx];
            wordle_input.input[idx] = 0;
            wordle_input.tile_animations[idx] = InputTileAnimation {
                remaining: WORDLE_INPUT_CLICK_DURATION,
                letter: removed_letter,
                kind: InputTileAnimationKind::Remove,
            };
        }
    }

    if is_key_pressed(KeyCode::Enter) {
        if wordle_input.cursor < 5 {
            return;
        }

        let Some(target_row) = wordle_game
            .curr_level
            .attempts
            .iter()
            .position(|attempt| attempt.word[0] == 0)
        else {
            return;
        };

        let guess = wordle_input.input;
        let progress = wordle_progress(guess, wordle_game.curr_level.solution);

        wordle_game.active_animation = Some(AttemptAnimation {
            guess,
            progress,
            target_row,
            phase: AttemptAnimationPhase::Translate,
            phase_elapsed: 0.,
        });

        wordle_input.input = [0; 5];
        wordle_input.cursor = 0;
        wordle_input.tile_animations = [InputTileAnimation::default(); 5];
    }
}

pub fn update_wordle_animation(wordle_game: &mut WordleGame) {
    let Some(animation) = wordle_game.active_animation.as_mut() else {
        return;
    };

    animation.phase_elapsed += get_frame_time();

    match animation.phase {
        AttemptAnimationPhase::Translate => {
            if animation.phase_elapsed >= WORDLE_TRANSLATE_DURATION {
                animation.phase = AttemptAnimationPhase::Flip;
                animation.phase_elapsed = 0.;
            }
        }
        AttemptAnimationPhase::Flip => {
            let total_flip_duration = WORDLE_FLIP_DURATION + WORDLE_FLIP_STAGGER * 4.;
            if animation.phase_elapsed >= total_flip_duration {
                animation.phase = AttemptAnimationPhase::Settle;
                animation.phase_elapsed = 0.;
            }
        }
        AttemptAnimationPhase::Settle => {
            if animation.phase_elapsed >= WORDLE_SETTLE_DURATION {
                let completed_attempt = Attempt {
                    word: animation.guess,
                    progress: animation.progress,
                };
                wordle_game.curr_level.attempts[animation.target_row] = completed_attempt;
                wordle_game.active_animation = None;
            }
        }
    }
}

pub fn update_wordle_input_animation(wordle_input: &mut WordleInput) {
    let frame_time = get_frame_time();

    for animation in &mut wordle_input.tile_animations {
        if animation.remaining <= 0. {
            continue;
        }

        animation.remaining = (animation.remaining - frame_time).max(0.);
        if animation.remaining == 0. {
            *animation = InputTileAnimation::default();
        }
    }
}
