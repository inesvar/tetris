//! Define the render function of [App].
use super::{App, RunningState, ViewState};
use crate::app::player::LocalPlayer;
use crate::app::remote::RemotePlayer;
use crate::app::ui::button::Button;
use crate::app::ui::interactive_widget_manager::InteractiveWidgetManager;
use crate::app::ui::key_input::KeyInput;
use crate::app::ui::text::Text;
use crate::app::ui::text_input::TextInput;
use crate::assets::Assets;
use crate::once;
use crate::settings::{BG_COLOR, DEFAULT_WINDOW_WIDTH};
use core_tetris::{Position, TetrisColor, TetrisGrid, TetrisPlayer, Tetromino};
use graphics::types::Matrix2d;
use graphics::Transformed;
use graphics::{Context, DrawState};
use include_assets::NamedArchive;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::RenderArgs;

pub trait RenderTetrisGame: RenderTetrisCore + RenderTetrisUi {
    type RenderArgs;
    fn render_app(&mut self, render_args: &RenderArgs, app: &App);
    fn render_local_player(&mut self, local_player: &LocalPlayer);
    fn render_remote_player(&mut self, remote_player: &RemotePlayer);
}

pub trait RenderTetrisUi {
    fn render_widget_manager(&mut self, manager: &InteractiveWidgetManager);
    fn render_text(&mut self, text: &Text);
    fn render_text_input(&mut self, input: &TextInput);
    fn render_key_input(&mut self, input: &KeyInput);
    fn render_button(&mut self, button: &Button);
}

pub trait RenderTetrisCore: RenderTetrisUi {
    fn render_player(&mut self, player: &TetrisPlayer);
    fn render_tetris_grid(&mut self, grid: &TetrisGrid);
    fn render_tetromino(&mut self, tetromino: &Tetromino);
    fn render_tetris_block(&mut self, ppsition: &Position, tetris_color: TetrisColor);
}

pub struct Piston2dOpenGlRenderer<'a> {
    pub gl: GlGraphics,
    pub assets: Assets<'a>,
    pub(super) transform: Matrix2d,
    pub(super) draw_state: DrawState,
    pub(super) elapsed_secs: f64,
}

impl<'a> Piston2dOpenGlRenderer<'a> {
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
                    self.render_local_player(player);
                    self.transform = self.transform.trans(DEFAULT_WINDOW_WIDTH as f64, 0.0);
                }
                for player in &app.remote_player {
                    self.render_remote_player(player);
                    self.transform = self.transform.trans(DEFAULT_WINDOW_WIDTH as f64, 0.0);
                }
            }
            _ => unreachable!(),
        }

        self.gl.draw_end();
    }

    fn render_remote_player(&mut self, remote_player: &RemotePlayer) {
        if !remote_player.received_first_screen() {
            return;
        }
        {
            let screen = remote_player.get_player();
            self.render_player(&screen);
        }
        once!("render was done");
    }

    fn render_local_player(&mut self, local_player: &LocalPlayer) {
        self.render_player(local_player.get_player());
    }
}
