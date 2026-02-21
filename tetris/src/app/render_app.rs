//! Define the render function of [App].
use super::{App, RunningState, ViewState};
use crate::app::player::{render_local_player, render_remote_player};
use crate::app::ui::{render_text, render_widget_manager};
use crate::assets::Assets;
use crate::settings::{BG_COLOR, DEFAULT_WINDOW_WIDTH};
use graphics::types::Matrix2d;
use graphics::Transformed;
use graphics::{Context, DrawState};
use include_assets::NamedArchive;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::RenderArgs;

pub struct Piston2dGraphicsArguments<'a> {
    pub gl: GlGraphics,
    pub assets: Assets<'a>,
    pub(super) transform: Matrix2d,
    pub(super) draw_state: DrawState,
    pub(super) elapsed_secs: f64,
}

impl<'a> Piston2dGraphicsArguments<'a> {
    pub fn new(gl_version: OpenGL, assets_archive: &'a NamedArchive) -> Self {
        let assets = Assets::new(assets_archive);

        Self {
            gl: GlGraphics::new(gl_version),
            assets,
            transform: Matrix2d::default(),
            draw_state: DrawState::default(),
            elapsed_secs: 0.0,
        }
    }

    fn update(&mut self, ctx: &Context, clock: f64) {
        self.transform = ctx.transform;
        self.draw_state = ctx.draw_state;
        self.elapsed_secs = clock;
    }
}

impl Piston2dGraphicsArguments<'_> {
    pub fn render(&mut self, args: &RenderArgs, app: &App) {
        let ctx = self.gl.draw_begin(args.viewport());

        self.update(&ctx, app.clock);

        // Clear the screen.
        graphics::clear(BG_COLOR, &mut self.gl);

        match &app.view_state {
            ViewState::MainMenu => {
                render_text(&app.title_text, self);
                render_widget_manager(&app.widget_manager[0], self)
            }
            ViewState::Settings => {
                render_text(&app.title_text, self);
                for widget_manager in &app.widget_manager {
                    render_widget_manager(widget_manager, self);
                }
            }
            ViewState::CreateRoom => {
                render_text(&app.title_text, self);
                render_widget_manager(&app.widget_manager[0], self)
            }
            ViewState::JoinRoom => {
                render_text(&app.title_text, self);
                render_widget_manager(&app.widget_manager[0], self)
            }
            a if a.is_game() => {
                render_widget_manager(&app.widget_manager[0], self);
                if app.running == RunningState::Running {
                    render_text(&app.title_text, self);
                } else if app.running == RunningState::NotRunning {
                    render_text(&app.restart_text, self);
                } else if app.running == RunningState::Paused {
                    render_text(&app.pause_text, self);
                } else if app.running == RunningState::Starting {
                    render_text(&app.title_text, self);
                }

                render_text(&app.timer_text, self);

                for player in &app.local_players {
                    render_local_player(player, self);
                    self.transform = self.transform.trans(DEFAULT_WINDOW_WIDTH as f64, 0.0);
                }
                for player in &app.remote_player {
                    render_remote_player(player, self);
                    self.transform = self.transform.trans(DEFAULT_WINDOW_WIDTH as f64, 0.0);
                }
            }
            _ => unreachable!(),
        }

        self.gl.draw_end();
    }
}
