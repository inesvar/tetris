use std::net::{TcpListener, TcpStream};
use std::thread;

use crate::assets::Assets;
use crate::local_player::Player;
use crate::settings::{NB_NEXT_TETROMINO, BLOCK_SIZE, NB_COLUMNS, NB_ROWS};
use crate::{
    tetris_grid::TetrisGrid, tetromino::Tetromino, local_player::LocalPlayer, player_screen::PlayerScreen,
};
use crate::circular_buffer::CircularBuffer;
use graphics::{Context, color, Transformed};
use opengl_graphics::GlGraphics;
use piston::{RenderArgs};

pub struct RemotePlayer {
    player_screen: PlayerScreen,
    listener: TcpListener,
}

impl RemotePlayer {
    pub fn new() -> Self {
        RemotePlayer {
            player_screen: PlayerScreen::new(),
            listener: TcpListener::bind("127.0.0.1:8000").unwrap(), // TODO : faire un unwrap or cohérent
        }
    }
    
    pub fn listen(&mut self) {
        for stream in self.listener.incoming() {
            let stream = stream.unwrap();
            let deserialized = serde_cbor::from_reader::<PlayerScreen, TcpStream>(stream).unwrap();
            self.player_screen = deserialized;
            self.render();
        }
    }

    pub fn render(&mut self, ctx: Context, gl: &mut GlGraphics, args: &RenderArgs, assets: &mut Assets) {
        let score_transform = ctx.transform.trans(0.0, 250.0);
        graphics::text::Text::new_color(color::WHITE, 16)
            .draw(
                format!("Score: {}", self.player_screen.score).as_str(),
                &mut assets.main_font,
                &ctx.draw_state,
                score_transform,
                gl,
            )
            .unwrap();

        self.player_screen.grid.render(args, &ctx, gl, assets);

        self.player_screen.active_tetromino
            .render(self.player_screen.grid.transform, &ctx, gl, assets);

        if let Some(saved) = self.player_screen.saved_tetromino {
            let transform = ctx.transform.trans(-70.0, 50.0);
            saved.render(transform, &ctx, gl, assets);
        }

        for i in 0..NB_NEXT_TETROMINO {
            let transform = ctx.transform.trans(BLOCK_SIZE * 16.0, 5.0 * BLOCK_SIZE + 4.0 * BLOCK_SIZE * i as f64);
            self.player_screen.fifo_next_tetromino.get(i).unwrap().render(transform, &ctx, gl, assets);
        }
    }
}