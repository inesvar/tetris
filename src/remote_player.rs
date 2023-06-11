use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

use crate::assets::Assets;
use crate::once;
use crate::player_screen::PlayerScreen;
use crate::settings::SERVER_IP;
use graphics::Context;
use graphics::math::Matrix2d;
use opengl_graphics::GlGraphics;

use crate::ui::text::Text;

pub struct RemotePlayer {
    screen: Arc<Mutex<PlayerScreen>>,
    initialized: Arc<Mutex<bool>>,
}

impl RemotePlayer {
    pub fn new() -> Self {
        let arc = Arc::new(Mutex::new(PlayerScreen::empty()));
        RemotePlayer {
            screen: arc,
            initialized: Arc::new(Mutex::new(false)),
        }
    }

    pub fn listen(&self) {
        let screen = Arc::clone(&self.screen);
        let initialized = Arc::clone(&self.initialized);
        let listener = TcpListener::bind(SERVER_IP).unwrap();
        thread::spawn(move || {
            for stream in listener.incoming() {
                let stream = stream.unwrap();
                let deserialized =
                    serde_cbor::from_reader::<PlayerScreen, TcpStream>(stream).unwrap();
                once!("unwrapped from {}", SERVER_IP);
                {
                    let mut screen = screen.lock().unwrap();
                    *screen = deserialized;
                    screen.ghost_tetromino = None;
                }
                {
                    if !*initialized.lock().unwrap() {
                        *initialized.lock().unwrap() = true;
                    }
                }
            }
        });
    }
        
    pub fn render(&self, transform: Matrix2d, ctx: &Context, gl: &mut GlGraphics, assets: &mut Assets) {
        if !*self.initialized.lock().unwrap() {
            return;
        }
        {
            let mut screen = self.screen.lock().unwrap();
            screen.render(transform, ctx, gl, assets);
        }
        once!("render was done");
    }

    pub fn get_lines_completed(&self) -> u64 {
        {
            let screen = self.screen.lock().unwrap();
            return screen.new_completed_lines;
        }
    }
}
