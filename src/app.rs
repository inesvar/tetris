



use crate::settings::*;
use crate::Assets;
use graphics::color;

use crate::ui::main_menu::MainMenu;
use crate::ui::text::Text;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::{MouseButton, RenderArgs, UpdateArgs};
use piston_window::Key;

pub enum ViewState {
    MainMenu,
    Settings,
    JoinRoom,
    CreateRoom,
    SinglePlayerGame,
    LocalMultiplayerGame,
    OnlineMultiplayerGame,
}

pub enum PlayerConfig {
    Local,
    Streamer,
    TwoLocal,
    TwoRemote,
    Viewer,
}

pub struct App<'a> {
    gl: GlGraphics,
    player_config: PlayerConfig,
    view_state: ViewState,
    assets: Assets<'a>,
    pub clock: f64,
    frame_counter: u64,
    running: bool,
    title_text: Text,
    restart_text: Text,
    timer_text: Text,

    pub cursor_position: [f64; 2],

    main_menu: MainMenu,
}

impl App<'_> {
    pub fn new(gl_version: OpenGL, player_config: PlayerConfig) -> App<'static> {
        let assets_folder = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets")
            .unwrap();

        let assets = Assets::new(assets_folder);

        let app = App {
            gl: GlGraphics::new(gl_version),
            player_config,
            view_state: ViewState::MainMenu,
            assets,
            title_text: Text::new(String::from("T"), 16, 180.0, 50.0, color::WHITE),
            restart_text: Text::new(
                String::from("Press R to (re)start"),
                16,
                180.0,
                50.0,
                color::WHITE,
            ),
            timer_text: Text::new(String::from("Elapsed: 0.0s"), 16, 80.0, 200.0, color::WHITE),
            clock: 0.0,
            frame_counter: 0,
            running: false,

            cursor_position: [0.0, 0.0],

            main_menu: MainMenu::new(),
        };
        app
    }

    pub(crate) fn render(&mut self, args: &RenderArgs) {
        self.gl.draw(args.viewport(), |ctx, gl| {
            // Clear the screen.
            graphics::clear(BG_COLOR, gl);

            match self.view_state {
                ViewState::MainMenu => {
                    self.title_text
                        .render(ctx.transform, &ctx, gl, &mut self.assets.tetris_font);
                    self.main_menu
                        .render(ctx.transform, &ctx, gl, &mut self.assets)
                }
                ViewState::Settings => {}
                _ => {
                    if self.running {
                        self.title_text.render(
                            ctx.transform,
                            &ctx,
                            gl,
                            &mut self.assets.tetris_font,
                        );
                    } else {
                        self.restart_text.render(
                            ctx.transform,
                            &ctx,
                            gl,
                            &mut self.assets.main_font,
                        );
                    }

                    self.timer_text
                        .set_text(format!("Elapsed: {:.2}s", self.clock));
                    self.timer_text
                        .render(ctx.transform, &ctx, gl, &mut self.assets.main_font);
                }
            }
        });
    }

    pub(crate) fn update(&mut self, args: &UpdateArgs) {
        // on ne fait pas d'update quand running == false
        if self.running {
            self.clock += args.dt;
            self.frame_counter = self.frame_counter.wrapping_add(1);
        }
    }

    pub fn handle_key_press(&mut self, _key: Key) {}

    pub fn handle_key_release(&mut self, _key: Key) {}

    pub fn handle_mouse_press(&mut self, button: MouseButton) {
        match self.view_state {
            ViewState::MainMenu => self
                .main_menu
                .handle_mouse_press(button, &self.cursor_position),
            _ => {}
        }

        if self.main_menu.create_single_player_game_button.is_pressed() {
            self.view_state = ViewState::SinglePlayerGame;
        }

        if self.main_menu.settings_button.is_pressed() {
            self.view_state = ViewState::Settings;
        }
    }

    pub fn handle_mouse_release(&mut self, button: MouseButton) {
        match self.view_state {
            ViewState::MainMenu => self
                .main_menu
                .handle_mouse_release(button, &self.cursor_position),
            _ => {}
        }
    }
}
