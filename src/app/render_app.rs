//! Defines the render function of [App].
use super::{App, RunningState, ViewState};
use crate::settings::{BG_COLOR, DEFAULT_WINDOW_WIDTH};
use crate::utils::formattings::format_seconds;
use graphics::Transformed;
use piston::RenderArgs;

impl App<'_> {
    pub fn render(&mut self, args: &RenderArgs) {
        self.gl.draw(args.viewport(), |ctx, gl| {
            // Clear the screen.
            graphics::clear(BG_COLOR, gl);

            match &self.view_state {
                ViewState::MainMenu => {
                    self.title_text
                        .render(ctx.transform, &ctx, gl, &mut self.assets.tetris_font);
                    self.widget_manager[0].render(ctx.transform, &ctx, gl, &mut self.assets)
                }
                ViewState::Settings => {
                    self.title_text
                        .render(ctx.transform, &ctx, gl, &mut self.assets.tetris_font);
                    for widget_manager in &mut self.widget_manager {
                        widget_manager.render(ctx.transform, &ctx, gl, &mut self.assets);
                    }
                }
                ViewState::CreateRoom => {
                    self.title_text
                        .render(ctx.transform, &ctx, gl, &mut self.assets.tetris_font);
                    self.widget_manager[0].render(ctx.transform, &ctx, gl, &mut self.assets)
                }
                ViewState::JoinRoom => {
                    self.title_text
                        .render(ctx.transform, &ctx, gl, &mut self.assets.tetris_font);
                    self.widget_manager[0].render(ctx.transform, &ctx, gl, &mut self.assets)
                }
                a if a.is_game() => {
                    if self.running == RunningState::Running {
                        self.title_text.render(
                            ctx.transform,
                            &ctx,
                            gl,
                            &mut self.assets.tetris_font,
                        );
                        self.timer_text.set_text(format_seconds(self.clock));
                    } else if self.running == RunningState::NotRunning {
                        self.restart_text.render(
                            ctx.transform,
                            &ctx,
                            gl,
                            &mut self.assets.main_font,
                        );
                        self.timer_text
                            .set_text(format!("Elapsed: {:.2}s", self.clock));
                    } else if self.running == RunningState::Paused {
                        self.pause_text
                            .render(ctx.transform, &ctx, gl, &mut self.assets.main_font);
                        self.timer_text
                            .set_text(format!("Elapsed: {:.2}s", self.clock));
                    } else if self.running == RunningState::Starting {
                        self.title_text.render(
                            ctx.transform,
                            &ctx,
                            gl,
                            &mut self.assets.tetris_font,
                        );
                        self.timer_text.set_text("Elapsed: 0.00s".to_string());
                    }

                    self.timer_text
                        .render(ctx.transform, &ctx, gl, &mut self.assets.main_font);

                    let mut nb_players = 0;
                    for player in &mut self.local_players {
                        player.render(
                            ctx.transform
                                .trans((DEFAULT_WINDOW_WIDTH * nb_players) as f64, 0.0),
                            &ctx,
                            gl,
                            &mut self.assets,
                        );
                        nb_players += 1;
                    }
                    for player in &mut self.remote_player {
                        player.render(
                            ctx.transform
                                .trans((DEFAULT_WINDOW_WIDTH * nb_players) as f64, 0.0),
                            &ctx,
                            gl,
                            &mut self.assets,
                        );
                        nb_players += 1;
                    }

                    self.widget_manager[0].render(ctx.transform, &ctx, gl, &mut self.assets)
                }
                _ => unreachable!(),
            }
        });
    }
}
