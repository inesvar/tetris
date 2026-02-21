//! Define the render function of [App].
use super::{App, RunningState, ViewState};
use crate::app::player::{render_local_player, render_remote_player};
use crate::assets::Assets;
use crate::settings::{BG_COLOR, DEFAULT_WINDOW_WIDTH};
use graphics::types::Matrix2d;
use graphics::DrawState;
use graphics::Transformed;
use include_assets::NamedArchive;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::RenderArgs;

pub struct Renderer<'a> {
    gl: GlGraphics,
    assets: Assets<'a>,
}

impl<'a> Renderer<'a> {
    pub fn new(gl_version: OpenGL, assets_archive: &'a NamedArchive) -> Self {
        let assets = Assets::new(assets_archive);

        Self {
            gl: GlGraphics::new(gl_version),
            assets,
        }
    }
}

pub(super) struct Piston2dGraphicsArguments<'short, 'long> {
    pub(super) transform: Matrix2d,
    pub(super) draw_state: DrawState,
    pub(super) gl: &'short mut GlGraphics,
    pub(super) assets: &'short mut Assets<'long>,
    pub(super) elapsed_secs: f64,
}

impl<'short, 'long> Piston2dGraphicsArguments<'short, 'long> {
    pub(in crate::app) fn new(
        transform: Matrix2d,
        draw_state: DrawState,
        gl: &'short mut GlGraphics,
        assets: &'short mut Assets<'long>,
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

impl Renderer<'_> {
    pub fn render(&mut self, args: &RenderArgs, app: &App) {
        self.gl.draw(args.viewport(), |ctx, gl| {
            // Clear the screen.
            graphics::clear(BG_COLOR, gl);

            let mut gl_ctx = Piston2dGraphicsArguments::new(
                ctx.transform,
                ctx.draw_state,
                gl,
                &mut self.assets,
                app.clock,
            );

            match &app.view_state {
                ViewState::MainMenu => {
                    app.title_text.render(&mut gl_ctx);
                    app.widget_manager[0].render(&mut gl_ctx)
                }
                ViewState::Settings => {
                    app.title_text.render(&mut gl_ctx);
                    for widget_manager in &app.widget_manager {
                        widget_manager.render(&mut gl_ctx);
                    }
                }
                ViewState::CreateRoom => {
                    app.title_text.render(&mut gl_ctx);
                    app.widget_manager[0].render(&mut gl_ctx)
                }
                ViewState::JoinRoom => {
                    app.title_text.render(&mut gl_ctx);
                    app.widget_manager[0].render(&mut gl_ctx)
                }
                a if a.is_game() => {
                    app.widget_manager[0].render(&mut gl_ctx);
                    if app.running == RunningState::Running {
                        app.title_text.render(&mut gl_ctx);
                    } else if app.running == RunningState::NotRunning {
                        app.restart_text.render(&mut gl_ctx);
                    } else if app.running == RunningState::Paused {
                        app.pause_text.render(&mut gl_ctx);
                    } else if app.running == RunningState::Starting {
                        app.title_text.render(&mut gl_ctx);
                    }

                    app.timer_text.render(&mut gl_ctx);

                    for player in &app.local_players {
                        render_local_player(player, &mut gl_ctx);
                        gl_ctx.transform = gl_ctx.transform.trans(DEFAULT_WINDOW_WIDTH as f64, 0.0);
                    }
                    for player in &app.remote_player {
                        render_remote_player(player, &mut gl_ctx);
                        gl_ctx.transform = gl_ctx.transform.trans(DEFAULT_WINDOW_WIDTH as f64, 0.0);
                    }
                }
                _ => unreachable!(),
            }
        });
    }
}
