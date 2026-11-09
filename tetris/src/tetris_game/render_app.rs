//! Define the render function of [App].
use super::RunningState;
use crate::app::{App, ViewState};
use crate::once;
use crate::settings::BG_COLOR;
use crate::tetris_game::player::LocalPlayer;
use crate::tetris_game::remote::RemotePlayer;
use crate::tetris_game::TetrisGame;
use core_tetris::render::RenderTetrisPlayer;
use graphics::Transformed;
use piston::RenderArgs;
use render_tetris::{Piston2dOpenGlRenderer, BLOCK_SIZE, DEFAULT_GRID_X};
use ui_tetris::{
    RenderTetrisUi, Text, DEFAULT_FONT_SIZE, DEFAULT_SCORE_TEXT_Y, DEFAULT_WINDOW_WIDTH, SILVER,
};

pub trait RenderTetrisApp: RenderTetrisPlayer + RenderTetrisUi {
    type RenderArgs;
    fn render_app(&mut self, render_args: &RenderArgs, app: &App);
    fn render_tetris_game(&mut self, game: &TetrisGame);
    fn render_local_player(
        &mut self,
        local_player: &LocalPlayer,
        state: RunningState,
        game_clock: f64,
    );
    fn render_remote_player(
        &mut self,
        remote_player: &RemotePlayer,
        state: RunningState,
        game_clock: f64,
    );
    fn render_player_score(&mut self, score: u32);
}

impl RenderTetrisApp for Piston2dOpenGlRenderer<'_> {
    type RenderArgs = RenderArgs;

    fn render_app(&mut self, args: &RenderArgs, app: &App) {
        let ctx = self.gl.draw_begin(args.viewport());

        self.update(&ctx, app.clock);

        // Clear the screen.
        graphics::clear(BG_COLOR, &mut self.gl);

        self.render_widget_manager(&app.widget_manager, &app.game.running);
        if matches!(app.view_state, ViewState::Game(_)) {
            self.render_tetris_game(&app.game);
        }

        self.gl.draw_end();
    }

    fn render_tetris_game(&mut self, game: &TetrisGame) {
        for player in &game.local_players {
            self.render_local_player(player, game.running, game.clock);
            self.transform = self.transform.trans(DEFAULT_WINDOW_WIDTH, 0.0);
        }
        for player in &game.remote_player {
            self.render_remote_player(player, game.running, game.clock);
            self.transform = self.transform.trans(DEFAULT_WINDOW_WIDTH, 0.0);
        }
    }

    fn render_local_player(
        &mut self,
        local_player: &LocalPlayer,
        state: RunningState,
        game_clock: f64,
    ) {
        self.render_player_score(local_player.score());
        self.display_player(local_player.get_player(), state, game_clock);
    }

    fn render_remote_player(
        &mut self,
        remote_player: &RemotePlayer,
        state: RunningState,
        game_clock: f64,
    ) {
        self.render_player_score(remote_player.score());

        if !remote_player.received_first_screen() {
            return;
        }
        {
            let screen = remote_player.get_player();
            self.display_player(&screen, state, game_clock);
        }
        once!("render was done");
    }

    fn render_player_score(&mut self, score: u32) {
        // TODO: score rendering should eventually be in render_tetris
        let score_text = Text::new(
            format!("Score: {}", score).as_str(),
            DEFAULT_FONT_SIZE,
            // the score is centered under the hold piece rectangle
            DEFAULT_GRID_X - 4.0 * BLOCK_SIZE, // 4.0 = 1.0 (margin between borders) + 1.0 (margin inside) + 2.0 (half TETROMINO_MAX_WIDTH)
            DEFAULT_SCORE_TEXT_Y,
            SILVER,
        );
        self.render_text(&score_text);
    }
}
