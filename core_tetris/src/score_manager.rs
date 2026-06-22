use super::{LineClear, TetrisCommand};
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Serialize, Deserialize)]
pub(super) struct ScoreManager {
    score: u32,
    back_to_back: bool,
    received_garbage_lines: u32,
}

#[derive(Default, PartialEq, Eq, Debug)]
pub(super) struct GarbageBalance {
    garbage: i32,
}

impl ScoreManager {
    pub(super) fn score(&self) -> u32 {
        self.score
    }

    pub(super) fn score_tetris_command(&mut self, command: TetrisCommand, nb_moves: u32) {
        self.score += command.score() * nb_moves;
    }

    /// Update the score accoding to `line_clear`, return the garbage balance.
    pub(super) fn score_line_clear(&mut self, line_clear: LineClear) -> GarbageBalance {
        if line_clear.stops_back_to_back_sequence() {
            self.back_to_back = false;
        }

        if self.back_to_back && line_clear.has_score_b2b_bonus() {
            self.score += line_clear.score() * 3 / 2;
        } else {
            self.score += line_clear.score();
        }

        let garbage_lines_to_send = if self.back_to_back && line_clear.has_garbage_b2b_bonus() {
            line_clear.nb_garbage_lines() + 1
        } else {
            line_clear.nb_garbage_lines()
        };

        if line_clear.begins_back_to_back_sequence() {
            self.back_to_back = true;
        }

        let garbage = GarbageBalance::new(self.received_garbage_lines, garbage_lines_to_send);

        self.received_garbage_lines = 0;

        garbage
    }

    pub(super) fn push_garbage(&mut self, nb_garbage_lines: u32) {
        self.received_garbage_lines += nb_garbage_lines;
    }
}

impl GarbageBalance {
    fn new(lines_received: u32, lines_to_send: u32) -> Self {
        Self {
            garbage: (lines_to_send - lines_received) as i32,
        }
    }

    pub(super) fn garbage_lines_to_add(&self) -> u32 {
        (-self.garbage).max(0) as u32
    }

    pub(super) fn garbage_lines_to_send(&self) -> u32 {
        self.garbage.max(0) as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn back_to_back_works() {
        let mut score_manager = ScoreManager::default();

        assert_eq!(score_manager.score(), 0);

        score_manager.score_line_clear(LineClear::Tetris);
        assert_eq!(score_manager.score(), 800);

        score_manager.score_line_clear(LineClear::TSpinDouble);
        assert_eq!(score_manager.score(), 2600);

        score_manager.score_line_clear(LineClear::TSpin);
        assert_eq!(score_manager.score(), 3000);

        score_manager.score_line_clear(LineClear::Tetris);
        assert_eq!(score_manager.score(), 4200);

        score_manager.score_line_clear(LineClear::TSpinSingle);
        assert_eq!(score_manager.score(), 5400);

        score_manager.score_line_clear(LineClear::Double);
        assert_eq!(score_manager.score(), 5700);
    }

    #[test]
    fn score_line_clear_returns_correct_garbage() {
        let mut score_manager = ScoreManager::default();

        let garbage = score_manager.score_line_clear(LineClear::Tetris);
        assert_eq!(garbage.garbage_lines_to_add(), 0);
        assert_eq!(garbage.garbage_lines_to_send(), 4);

        score_manager.push_garbage(3);
        let garbage = score_manager.score_line_clear(LineClear::None);
        assert_eq!(garbage.garbage_lines_to_add(), 3);
        assert_eq!(garbage.garbage_lines_to_send(), 0);

        score_manager.push_garbage(4);
        let garbage = score_manager.score_line_clear(LineClear::TSpinDouble);
        assert_eq!(garbage.garbage_lines_to_add(), 0);
        assert_eq!(garbage.garbage_lines_to_send(), 0);

        score_manager.push_garbage(2);
        score_manager.push_garbage(4);
        let garbage = score_manager.score_line_clear(LineClear::Triple);
        assert_eq!(garbage.garbage_lines_to_add(), 4);
        assert_eq!(garbage.garbage_lines_to_send(), 0);

        score_manager.push_garbage(1);
        score_manager.push_garbage(1);
        let garbage = score_manager.score_line_clear(LineClear::TSpinTriple);
        assert_eq!(garbage.garbage_lines_to_add(), 0);
        assert_eq!(garbage.garbage_lines_to_send(), 4);
    }
}
