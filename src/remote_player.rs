use std::net::{TcpListener, TcpStream};
use std::thread;
use std::sync::{Mutex, Arc};

use crate::assets::Assets;
use crate::settings::{NB_NEXT_TETROMINO, BLOCK_SIZE};
use crate::{
    player_screen::PlayerScreen,
};
use graphics::{Context, color, Transformed};
use opengl_graphics::GlGraphics;

use crate::ui::text::Text;

pub struct RemotePlayer {
    update_screen: Arc<Mutex<PlayerScreen>>,
    render_screen: Arc<Mutex<PlayerScreen>>,
    fresh: Arc<Mutex<bool>>,
}

impl RemotePlayer {
    pub fn new() -> Self {
        let arc = Arc::new(Mutex::new(PlayerScreen::new()));
        let arc2 = Arc::clone(&arc);
        RemotePlayer {
            update_screen: arc,
            render_screen: arc2,
            fresh: Arc::new(Mutex::new(false)),
        }
    }

    pub fn listen(&self) {
        let screen = Arc::clone(&self.update_screen);
        let fresh = Arc::clone(&self.fresh);
        let listener = TcpListener::bind("127.0.0.1:16000").unwrap();
        thread::spawn(move || {
            for stream in listener.incoming() {
                let stream = stream.unwrap();
                let deserialized = serde_cbor::from_reader::<PlayerScreen, TcpStream>(stream).unwrap();
                println!("unwrapped!!!");

                {
                    let mut screen = screen.lock().unwrap();
                    *screen = deserialized;
                }
                {
                    let mut fresh = fresh.lock().unwrap();
                    *fresh = true;
                }
            }
        });
    }

    pub fn render(&self, ctx: Context, gl: &mut GlGraphics, assets: &mut Assets) {
        if !*self.fresh.lock().unwrap() {
            return;
        } else {
            {
                let mut fresh = self.fresh.lock().unwrap();
                *fresh = false;
            }
        }
        println!("got to render");

        let mut render_screen = self.render_screen.lock().unwrap();

        let score_text = Text::new(format!("Score: {}", render_screen.score), 16, 0.0, 250.0, color::WHITE);
        score_text.render(ctx.transform, &ctx, gl, &mut assets.main_font);

        render_screen.grid.render(ctx.transform, &ctx, gl, assets);

        render_screen.active_tetromino.render(render_screen.grid.transform, &ctx, gl, assets);

        if let Some(saved) = render_screen.saved_tetromino {
            let transform = render_screen.grid.transform.trans(-100.0 - (saved.center.x as f64 * BLOCK_SIZE), 50.0);
            saved.render(transform, &ctx, gl, assets);
        }

        for i in 0..NB_NEXT_TETROMINO {
            let transform = ctx.transform.trans(BLOCK_SIZE * 16.0, 5.0 * BLOCK_SIZE + 4.0 * BLOCK_SIZE * i as f64);
            self.render_screen.lock().unwrap().fifo_next_tetromino.get(i).unwrap().render(transform, &ctx, gl, assets);
        }
    }
}