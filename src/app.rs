use crate::local_player::{KeyPress, LocalPlayer};
use std::cell::RefCell;
use std::ops::{Deref, DerefMut};

use crate::graphics::Transformed;
use crate::remote_player::RemotePlayer;
use crate::settings::*;
use crate::Assets;
use graphics::color;
use piston::event_id::TEXT;

use crate::ui::interactive_widget_manager::ButtonType::{
    BackToMainMenu, NewSinglePlayerGame, Pause, Settings, self,
};
use crate::ui::interactive_widget_manager::InteractiveWidgetManager;
use crate::ui::text::Text;
use crate::ui::text_input::TextInput;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::{MouseButton, RenderArgs, UpdateArgs};
use piston_window::Key;
#[derive(Debug)]
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
    local_players: Vec<LocalPlayer>,
    remote_players: Vec<RemotePlayer>,
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

    widget_manager: InteractiveWidgetManager,
}

impl App<'_> {
    pub fn new(gl_version: OpenGL, player_config: PlayerConfig) -> App<'static> {
        let assets_folder = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets")
            .unwrap();

        let local_player: LocalPlayer;
        let remote_player: RemotePlayer;
        let players: Vec<LocalPlayer>;
        let rem_players: Vec<RemotePlayer>;
        let seed: u64 = SEED;

        match player_config {
            PlayerConfig::Local => {
                local_player = LocalPlayer::new(seed, false);
                players = vec![local_player];
                rem_players = vec![];
            }
            PlayerConfig::Streamer => {
                local_player = LocalPlayer::new(seed, true);
                players = vec![local_player];
                rem_players = vec![];
            }
            PlayerConfig::Viewer => {
                remote_player = RemotePlayer::new();
                players = vec![];
                rem_players = vec![remote_player];
            }
            PlayerConfig::TwoRemote => {
                local_player = LocalPlayer::new(seed, true);
                players = vec![local_player];
                remote_player = RemotePlayer::new();
                rem_players = vec![remote_player];
            }
            _ => todo!(),
        }

        let assets = Assets::new(assets_folder);

        let app = App {
            gl: GlGraphics::new(gl_version),
            local_players: players,
            remote_players: rem_players,
            player_config,
            view_state: ViewState::MainMenu,
            assets,
            title_text: Text::new(
                "T",
                16,
                DEFAULT_WINDOW_WIDTH as f64 * 27.0 / 65.0,
                DEFAULT_TITLE_Y,
                TEXT_COLOR,
            ),
            restart_text: Text::new(
                "Press R to (re)start",
                22,
                DEFAULT_WINDOW_WIDTH as f64 / 2.0,
                DEFAULT_TITLE_Y,
                TEXT_COLOR,
            ),
            timer_text: Text::new(
                "Elapsed: 0.0s",
                16,
                DEFAULT_GRID_X - 4.0 * BLOCK_SIZE,
                DEFAULT_SCORE_TEXT_Y + 1.5 * BLOCK_SIZE,
                TEXT_COLOR,
            ),
            clock: 0.0,
            frame_counter: 0,
            running: false,

            cursor_position: [0.0, 0.0],

            widget_manager: InteractiveWidgetManager::new_main_menu(),
        };

        if let PlayerConfig::Viewer = app.player_config {
            app.remote_players[0].listen()
        } else if let PlayerConfig::TwoRemote = app.player_config {
            app.remote_players[0].listen()
        }
        app
    }

    pub(crate) fn render(&mut self, args: &RenderArgs) {
        self.gl.draw(args.viewport(), |ctx, gl| {
            // Clear the screen.
            graphics::clear(BG_COLOR, gl);

            for player in &self.local_players {
                if player.game_over() {
                    self.running = false;
                }
            }

            match self.view_state {
                ViewState::MainMenu => {
                    self.title_text
                        .render(ctx.transform, &ctx, gl, &mut self.assets.tetris_font);
                    self.widget_manager
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

                    let mut nb_players = 0;
                    for player in &mut self.local_players {
                        player.render(
                            ctx.transform
                                .trans((DEFAULT_WINDOW_HEIGHT * nb_players) as f64, 0.0),
                            &ctx,
                            gl,
                            &mut self.assets,
                        );
                        nb_players += 1;
                    }
                    for player in &mut self.remote_players {
                        player.render(
                            ctx.transform
                                .trans((DEFAULT_WINDOW_HEIGHT * nb_players) as f64, 0.0),
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

    pub(crate) fn update(&mut self, args: &UpdateArgs, gravity: u64, freeze: u64) {
        // on ne fait pas d'update quand running == false
        if self.running {
            self.clock += args.dt;
            self.frame_counter = self.frame_counter.wrapping_add(1);
            if let PlayerConfig::TwoRemote = self.player_config {
                for player in &mut self.local_players {
                    let completed_lines = self.remote_players[0].get_lines_completed();
                    if completed_lines != 0 {
                        println!("the adversary completed {} lines", completed_lines);
                        player.add_garbage(completed_lines);
                    }
                }
            }
            for player in &mut self.local_players {
                player.update(self.frame_counter, gravity, freeze);
            }
        }
    }

    pub fn handle_text_input(&mut self, input: &String) {
        match self.view_state {
            ViewState::MainMenu => self.widget_manager.handle_text_input(input),
            _ => {}
        }
    }

    pub fn handle_key_press(&mut self, key: Key) {
        let mut restart = false;
        for player in &mut self.local_players {
            match player.handle_key_press(key, self.running) {
                KeyPress::Restart => {
                    restart = true;
                }
                KeyPress::Other => {}
            }
        }
        if restart {
            self.running = true;
            self.clock = 0.0;
            for player in &mut self.local_players {
                player.restart();
            }
        }

        match self.view_state {
            ViewState::MainMenu => self.widget_manager.handle_key_press(key),
            _ => {}
        }
    }

    pub fn handle_key_release(&mut self, key: Key) {
        if self.running {
            for player in &mut self.local_players {
                player.handle_key_release(key);
            }
        }
    }

    pub fn handle_mouse_press(&mut self, button: MouseButton) {
        let mut result;
        result = self
            .widget_manager
            .handle_mouse_press(button, &self.cursor_position);
        println!("result is {:?}", result);
        match result {
            ButtonType::Pause => {}
            ButtonType::Nothing => {}
            ButtonType::BackToMainMenu => self.set_view(ViewState::MainMenu),
            ButtonType::Settings => self.set_view(ViewState::Settings),
            ButtonType::NewSinglePlayerGame => self.set_view(ViewState::SinglePlayerGame),
            _ => {},
        }
    }

    pub fn handle_mouse_release(&mut self, button: MouseButton) {
        match self.view_state {
            ViewState::MainMenu => self
                .widget_manager
                .handle_mouse_release(button, &self.cursor_position),
            _ => {}
        }
    }

    fn set_view(&mut self, view_state: ViewState) {
        println!("set view was called {:?}", view_state);
        self.view_state = view_state;
        match self.view_state {
            ViewState::MainMenu => self.widget_manager = InteractiveWidgetManager::new_main_menu(),
            ViewState::SinglePlayerGame => {
                self.widget_manager = InteractiveWidgetManager::new_single_player_game()
            }
            _ => self.widget_manager = InteractiveWidgetManager::new_empty(),
        }
    }
}
