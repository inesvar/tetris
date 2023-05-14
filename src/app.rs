use crate::local_player::{LocalPlayer, Player, KeyPress};
use crate::remote_player::RemotePlayer;
use crate::Assets;
use crate::settings::*;
use graphics::color;
use graphics::Transformed;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::{RenderArgs, UpdateArgs};
use piston_window::Key;

pub enum PlayerConfig {
    OneLocal,
    TwoLocal,
    OneLocalOneRemote,
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
    pub fn new(gl_version: OpenGL) -> App<'static> {
        let assets_folder = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets")
            .unwrap();

        let player1 = LocalPlayer::new();
        let mut players: Vec<LocalPlayer> = vec!();
        players.push(player1);
        let rem_players: Vec<RemotePlayer> = vec!();

        App {
            gl: GlGraphics::new(gl_version),
            local_players: players,
            remote_players: rem_players,
            player_config: PlayerConfig::OneLocal,
            assets: Assets::new(assets_folder),
            clock: 0.0,
            frame_counter: 0,
            running: true,
        }
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
        if !self.running {
            return;
        } else {
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
                KeyPress::Restart => {restart = true;},
                KeyPress::Other => {},
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
