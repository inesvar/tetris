//! Define the render function of [App].
use super::{App, RunningState, ViewState};
use crate::assets::Assets;
use crate::settings::{BG_COLOR, DEFAULT_WINDOW_WIDTH};
use crate::utils::formattings::format_seconds;
use graphics::types::Matrix2d;
use graphics::DrawState;
use graphics::Transformed;
use opengl_graphics::GlGraphics;
use piston::RenderArgs;

pub(super) struct Piston2dGraphicsArguments<'short> {
    pub(super) transform: Matrix2d,
    pub(super) draw_state: DrawState,
    pub(super) gl: &'short mut GlGraphics,
    pub(super) assets: &'short mut Assets,
    pub(super) elapsed_secs: f64,
}

impl<'short> Piston2dGraphicsArguments<'short> {
    pub(in crate::app) fn new(
        transform: Matrix2d,
        draw_state: DrawState,
        gl: &'short mut GlGraphics,
        assets: &'short mut Assets,
        elapsed_secs: f64,
    ) -> Self {
        Self {
            transform,
            draw_state,
            gl,
            assets,
            elapsed_secs,
        }
    }
}

#[allow(dead_code)]
/// Draw tetris objects.
pub(super) trait Render<GlCtx> {
    fn render(&self, graphics_args: &mut GlCtx);
}

impl App {
    pub fn render(&mut self, args: &RenderArgs) {
        self.gl.draw(args.viewport(), |ctx, gl| {
            // Clear the screen.
            graphics::clear(BG_COLOR, gl);

            let mut gl_ctx = Piston2dGraphicsArguments::new(
                ctx.transform,
                ctx.draw_state,
                gl,
                &mut self.assets,
                self.clock,
            );

            match &self.view_state {
                ViewState::MainMenu => {
                    self.widget_manager[0].render(&mut gl_ctx)
                }
                ViewState::Settings => {
                    for widget_manager in &mut self.widget_manager {
                        widget_manager.render(&mut gl_ctx);
                    }
                }
                ViewState::CreateRoom => {
                    self.widget_manager[0].render(&mut gl_ctx)
                }
                ViewState::JoinRoom => {
                    self.widget_manager[0].render(&mut gl_ctx)
                }
                a if a.is_game() => {
                    self.widget_manager[0].render(&mut gl_ctx);
                    if self.running == RunningState::Running {
                        self.timer_text.set_text(format_seconds(self.clock));
                    } else if self.running == RunningState::NotRunning {
                        self.timer_text
                            .set_text(format!("Elapsed: {:.2}s", self.clock));
                    } else if self.running == RunningState::Paused {
                        self.timer_text
                            .set_text(format!("Elapsed: {:.2}s", self.clock));
                    } else if self.running == RunningState::Starting {
                        self.timer_text.set_text("Elapsed: 0.00s".to_string());
                    }

                    for player in &mut self.local_players {
                        player.render(&mut gl_ctx);
                        gl_ctx.transform = gl_ctx.transform.trans(DEFAULT_WINDOW_WIDTH as f64, 0.0);
                    }
                    for player in &mut self.remote_player {
                        player.render(&mut gl_ctx);
                        gl_ctx.transform = gl_ctx.transform.trans(DEFAULT_WINDOW_WIDTH as f64, 0.0);
                    }
                }
                _ => unreachable!(),
            }
        });
    }
}
