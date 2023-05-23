use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

use crate::assets::Assets;
use crate::once;
use crate::player_screen::PlayerScreen;
use crate::settings::STREAMER_IP;
use graphics::Context;
use opengl_graphics::GlGraphics;
use piston::{RenderArgs};
use crate::ui::text::Text;

pub struct RemotePlayer {
    update_screen: Arc<Mutex<PlayerScreen>>,
    render_screen: Arc<Mutex<PlayerScreen>>,
    fresh: Arc<Mutex<bool>>,
}

impl RemotePlayer {
    pub fn new() -> Self {
        let arc = Arc::new(Mutex::new(PlayerScreen::empty()));
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
        let listener = TcpListener::bind(STREAMER_IP).unwrap();
        thread::spawn(move || {
            for stream in listener.incoming() {
                let stream = stream.unwrap();
                let deserialized =
                    serde_cbor::from_reader::<PlayerScreen, TcpStream>(stream).unwrap();
                once!("unwrapped from {}", STREAMER_IP);
                {
                    let mut screen = screen.lock().unwrap();
                    *screen = deserialized;
                    screen.ghost_tetromino = None;
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
        {
            let mut screen = self.render_screen.lock().unwrap();
            screen.render(ctx, gl, assets);
        }
        once!("render was done");
    }
}
