use crate::local_player::{KeyPress, LocalPlayer, Player, self};
use crate::player_screen::PlayerScreen;
use crate::remote_player::RemotePlayer;
use crate::settings::*;
use crate::Assets;
use graphics::color;
use graphics::Transformed;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::{RenderArgs, UpdateArgs};
use piston_window::Key;

pub enum PlayerConfig {
    OneLocal,
    TwoLocal,
    OneLocalOneRemote,
    OneRemote,
}

pub struct App<'a> {
    gl: GlGraphics,
    local_players: Vec<LocalPlayer>,
    remote_players: Vec<RemotePlayer>,
    player_config: PlayerConfig,
    assets: Assets<'a>,
    clock: f64,
    frame_counter: u64,
    running: bool,
}

impl App<'_> {
    pub fn new(gl_version: OpenGL, player_config: PlayerConfig) -> App<'static> {
        let assets_folder = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets")
            .unwrap();
        
        let mut local_player: LocalPlayer;
        let mut remote_player: RemotePlayer;
        let mut players: Vec<LocalPlayer>;
        let mut rem_players: Vec<RemotePlayer>;

        match player_config {
            PlayerConfig::OneLocal => {
                local_player = LocalPlayer::new();
                players = vec![local_player];
                rem_players = vec![];
            }
            PlayerConfig::OneRemote => {
                remote_player = RemotePlayer::new();
                players = vec![];
                rem_players = vec![remote_player];
            }
            _ => todo!(),
        }

        let app = App {
            gl: GlGraphics::new(gl_version),
            local_players: players,
            remote_players: rem_players,
            player_config,
            assets: Assets::new(assets_folder),
            clock: 0.0,
            frame_counter: 0,
            running: true,
        };
        match app.player_config {
            PlayerConfig::OneRemote => app.remote_players[0].listen(),
            _ => {},
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
            if self.running {
                let title_transform = ctx.transform.trans(180.0, 50.0);
                graphics::text::Text::new_color(color::WHITE, 16)
                    .draw(
                        "T",
                        &mut self.assets.tetris_font,
                        &ctx.draw_state,
                        title_transform,
                        gl,
                    )
                    .unwrap();
            } else {
                let restart_transform = ctx.transform.trans(180.0, 50.0);
                graphics::text::Text::new_color(color::WHITE, 16)
                    .draw(
                        "Press R to restart",
                        &mut self.assets.main_font,
                        &ctx.draw_state,
                        restart_transform,
                        gl,
                    )
                    .unwrap();
            }

            let timer_transform = ctx.transform.trans(0.0, 200.0);
            graphics::text::Text::new_color(color::WHITE, 16)
                .draw(
                    format!("Elapsed: {:.2}s", self.clock).as_str(),
                    &mut self.assets.main_font,
                    &ctx.draw_state,
                    timer_transform,
                    gl,
                )
                .unwrap();

            for player in &mut self.local_players {
                player.render(ctx, gl, args, &mut self.assets);
            }
            for player in &mut self.remote_players {
                player.render(ctx, gl, args, &mut self.assets);
            }
        });
    }

    pub(crate) fn update(&mut self, args: &UpdateArgs) {
        // on ne fait pas d'update quand running == false
        if self.running {
            self.clock += args.dt;
            self.frame_counter = self.frame_counter.wrapping_add(1);
            for player in &mut self.local_players {
                player.update(self.frame_counter);
            }
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
    }

    pub fn handle_key_release(&mut self, key: Key) {
        if self.running {
            for player in &mut self.local_players {
                player.handle_key_release(key);
            }
        }
    }
}
