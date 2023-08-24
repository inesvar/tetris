//! Defines the render function of [App].
use super::{App, RunningState, ViewState};
use crate::settings::{BG_COLOR, DEFAULT_WINDOW_WIDTH};
use graphics::Transformed;
use piston::RenderArgs;

impl App<'_> {
    pub fn render(&mut self, args: &RenderArgs) {
        self.gl.draw(args.viewport(), |ctx, gl| {
            // Clear the screen.
            graphics::clear(BG_COLOR, gl);

            match self.view_state {
                ViewState::MainMenu => {
                    self.title_text
                        .render(ctx.transform, &ctx, gl, &mut self.assets.tetris_font);
                    self.widget_manager
                        .render(ctx.transform, &ctx, gl, &mut self.assets)
                }
                ViewState::Settings => {
                    self.title_text
                        .render(ctx.transform, &ctx, gl, &mut self.assets.tetris_font);
                    self.widget_manager
                        .render(ctx.transform, &ctx, gl, &mut self.assets)
                }
                _ => {
                    if self.running == RunningState::Running {
                        self.title_text.render(
                            ctx.transform,
                            &ctx,
                            gl,
                            &mut self.assets.tetris_font,
                        );
                    } else if self.running == RunningState::NotRunning {
                        self.restart_text.render(
                            ctx.transform,
                            &ctx,
                            gl,
                            &mut self.assets.main_font,
                        );
                    } else {
                        self.pause_text
                            .render(ctx.transform, &ctx, gl, &mut self.assets.main_font);
                    }

                    self.timer_text
                        .set_text(format!("Elapsed: {:.2}s", self.clock));
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
                    for player in &mut self.remote_players {
                        player.render(
                            ctx.transform
                                .trans((DEFAULT_WINDOW_WIDTH * nb_players) as f64, 0.0),
                            &ctx,
                            gl,
                            &mut self.assets,
                        );
                        nb_players += 1;
                    }

                    self.widget_manager
                        .render(ctx.transform, &ctx, gl, &mut self.assets)
                }
            }
        });
    }
}
