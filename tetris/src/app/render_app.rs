//! Define the render function of [App].
use super::{App, RunningState, ViewState};
use crate::app::player::LocalPlayer;
use crate::app::remote::RemotePlayer;
use crate::once;
use crate::settings::{BG_COLOR, DEFAULT_SCORE_TEXT_Y};
use core_tetris::render::RenderTetrisPlayer;
use graphics::Transformed;
use piston::RenderArgs;
use render_tetris::Piston2dOpenGlRenderer;
use render_tetris::{BLOCK_SIZE, DEFAULT_GRID_X};
use ui_tetris::{RenderTetrisUi, Text, DEFAULT_FONT_SIZE, DEFAULT_WINDOW_WIDTH, SILVER};

pub trait RenderTetrisGame: RenderTetrisPlayer + RenderTetrisUi {
    type RenderArgs;
    fn render_app(&mut self, render_args: &RenderArgs, app: &App);
    fn render_local_player(&mut self, local_player: &LocalPlayer, state: RunningState);
    fn render_remote_player(&mut self, remote_player: &RemotePlayer, state: RunningState);
}

impl RenderTetrisGame for Piston2dOpenGlRenderer<'_> {
    type RenderArgs = RenderArgs;

    fn render_app(&mut self, args: &RenderArgs, app: &App) {
        let ctx = self.gl.draw_begin(args.viewport());

        self.update(&ctx, app.clock);

        // Clear the screen.
        graphics::clear(BG_COLOR, &mut self.gl);

        match &app.view_state {
            ViewState::MainMenu | ViewState::CreateRoom | ViewState::JoinRoom => {
                self.render_text(&app.title_text);
                self.render_widget_manager(&app.widget_manager[0])
            }
            ViewState::Settings => {
                self.render_text(&app.title_text);
                for widget_manager in &app.widget_manager {
                    self.render_widget_manager(widget_manager);
                }
            }
            a if a.is_game() => {
                self.render_widget_manager(&app.widget_manager[0]);
                match app.running {
                    RunningState::Running | RunningState::Starting => {
                        self.render_text(&app.title_text)
                    }
                    RunningState::NotRunning => self.render_text(&app.restart_text),
                    RunningState::Paused => self.render_text(&app.pause_text),
                }
                self.render_text(&app.timer_text);

                for player in &app.local_players {
                    // TODO: score rendering should eventually be in render_tetris
                    let score_text = Text::new(
                        format!("Score: {}", player.score()).as_str(),
                        DEFAULT_FONT_SIZE,
                        // the score is centered under the hold piece rectangle
                        DEFAULT_GRID_X - 4.0 * BLOCK_SIZE, // 4.0 = 1.0 (margin between borders) + 1.0 (margin inside) + 2.0 (half TETROMINO_MAX_WIDTH)
                        DEFAULT_SCORE_TEXT_Y,
                        SILVER,
                    );
                    self.render_text(&score_text);

                    self.render_local_player(player, app.running);
                    self.transform = self.transform.trans(DEFAULT_WINDOW_WIDTH, 0.0);
                }
                for player in &app.remote_player {
                    let score_text = Text::new(
                        format!("Score: {}", player.score()).as_str(),
                        DEFAULT_FONT_SIZE,
                        // the score is centered under the hold piece rectangle
                        DEFAULT_GRID_X - 4.0 * BLOCK_SIZE, // 4.0 = 1.0 (margin between borders) + 1.0 (margin inside) + 2.0 (half TETROMINO_MAX_WIDTH)
                        DEFAULT_SCORE_TEXT_Y,
                        SILVER,
                    );
                    self.render_text(&score_text);

                    self.render_remote_player(player, app.running);
                    self.transform = self.transform.trans(DEFAULT_WINDOW_WIDTH, 0.0);
                }
            }
            _ => unreachable!(),
        }

        self.gl.draw_end();
    }

    fn render_remote_player(&mut self, remote_player: &RemotePlayer, state: RunningState) {
        if !remote_player.received_first_screen() {
            return;
        }
        {
            let screen = remote_player.get_player();
            self.display_player(&screen, state);
        }
        once!("render was done");
    }

    fn render_local_player(&mut self, local_player: &LocalPlayer, state: RunningState) {
        self.display_player(local_player.get_player(), state);
    }
}
