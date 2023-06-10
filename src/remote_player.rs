use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

use crate::assets::Assets;
use crate::once;
use crate::player_screen::PlayerScreen;
use crate::settings::SERVER_IP;
use graphics::Context;
use opengl_graphics::GlGraphics;

use crate::ui::text::Text;

pub struct RemotePlayer {
    update_screen: Arc<Mutex<PlayerScreen>>,
    render_screen: Arc<Mutex<PlayerScreen>>,
    initialized: Arc<Mutex<bool>>,
}

impl RemotePlayer {
    pub fn new() -> Self {
        let arc = Arc::new(Mutex::new(PlayerScreen::empty()));
        let arc2 = Arc::clone(&arc);
        RemotePlayer {
            update_screen: arc,
            render_screen: arc2,
            initialized: Arc::new(Mutex::new(false)),
        }
    }

    pub fn listen(&self) {
        let screen = Arc::clone(&self.update_screen);
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

    pub fn render(&self, ctx: Context, gl: &mut GlGraphics, assets: &mut Assets) {
        if !*self.initialized.lock().unwrap() {
            return;
        }
        {
            let mut screen = self.render_screen.lock().unwrap();
            screen.render(ctx, gl, assets);
        }
        once!("render was done");
    }
}
