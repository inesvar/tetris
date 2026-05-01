//! Defines `trait` [RenderTetrisPlayer] and `enum` [RunningState].

use super::{Position, TetrisColor, TetrisGrid, TetrisPlayer, Tetromino};

/// State of the game.
///
/// Can be used to customize [TetrisPlayer] rendering through [RenderTetrisPlayer] functions.
#[allow(missing_docs)]
#[derive(PartialEq, Clone, Copy)]
pub enum RunningState {
    Running,
    Paused,
    NotRunning,
    Starting,
}

/// Rendering a [TetrisPlayer].
///
/// All methods have default implementations except `render_tetris_block`.
/// However `display_hold_queue` and `display_next_queue` have empty default
/// implementations: except in the case of a minimal implementation, it's
/// recommended to provide a custom implementation.
///
/// In order to display the [TetrisPlayer::score], you might want to implement
/// a `render_text` method.
#[allow(missing_docs)]
pub trait RenderTetrisPlayer {
    fn display_player(&mut self, player: &TetrisPlayer, state: RunningState) {
        self.display_tetris_grid(player, state);
        self.display_tetromino_in_play(player, state);
        self.display_hold_queue(player, state);
        self.display_next_queue(player, state);
    }

    fn display_tetris_grid(&mut self, player: &TetrisPlayer, _state: RunningState) {
        self.render_tetris_grid(player.grid());
    }

    fn display_hold_queue(&mut self, _player: &TetrisPlayer, _state: RunningState) {}
    fn display_next_queue(&mut self, _player: &TetrisPlayer, _state: RunningState) {}

    fn display_tetromino_in_play(&mut self, player: &TetrisPlayer, _state: RunningState) {
        self.render_tetromino(player.tetromino_in_play());
    }

    fn render_tetris_grid(&mut self, grid: &TetrisGrid) {
        for position in grid.positions() {
            if let Some(tetris_color) = grid[&position] {
                self.render_tetris_block(&position, tetris_color);
            }
        }
    }
    fn render_tetromino(&mut self, tetromino: &Tetromino) {
        for block in tetromino.blocks() {
            self.render_tetris_block(block, tetromino.color());
        }
    }
    fn render_tetris_block(&mut self, position: &Position, tetris_color: TetrisColor);
}
