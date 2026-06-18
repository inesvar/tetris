use super::{LineClear, TetrisCommand};
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Serialize, Deserialize)]
pub(super) struct ScoreManager {
    score: u32,
    back_to_back: bool,
}

impl ScoreManager {
    pub(super) fn score(&self) -> u32 {
        self.score
    }

    pub(super) fn score_tetris_command(&mut self, command: TetrisCommand, nb_moves: u32) {
        self.score += command.score() * nb_moves;
    }

    pub(super) fn score_line_clear(&mut self, line_clear: LineClear) {
        if line_clear.stops_back_to_back_sequence() {
            self.back_to_back = false;
        }

        if self.back_to_back && line_clear.has_back_to_back_bonus() {
            self.score += line_clear.score() * 3 / 2;
        } else {
            self.score += line_clear.score();
        }

        if line_clear.begins_back_to_back_sequence() {
            self.back_to_back = true;
        }
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
}
