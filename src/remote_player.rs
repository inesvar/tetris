use crate::assets::Assets;
use crate::once;
use crate::player_screen::PlayerScreen;
use crate::settings::SERVER_IP;
use graphics::math::Matrix2d;
use graphics::Context;
use opengl_graphics::GlGraphics;
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

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
        // building a second RemotePlayer that points to the same pointees than self
        // this is necessary because self can't be moved out to another thread
        let screen = Arc::clone(&self.screen);
        let initialized = Arc::clone(&self.initialized);
        let self_for_listener = RemotePlayer {
            screen: screen,
            initialized,
        };
        // creating a listener in a separate thread
        let listener = TcpListener::bind(SERVER_IP).unwrap();
        thread::spawn(move || {
            // for each incoming message
            for stream in listener.incoming() {
                let stream = stream.unwrap();
                let deserialized =
                    serde_cbor::from_reader::<PlayerScreen, TcpStream>(stream).unwrap();
                once!("unwrapped from {}", SERVER_IP);
                {
                    let mut screen = self_for_listener.screen.lock().unwrap();
                    // if the new_completed_lines haven't been read yet, ensure it's not rewritten
                    if screen.new_completed_lines != 0 {
                        let a = screen.new_completed_lines;
                        *screen = deserialized;
                        screen.new_completed_lines = a;
                    } else {
                        *screen = deserialized;
                    }
                    // erase the ghost tetromino so there's no confusion for the player on which grid is his
                    screen.ghost_tetromino = None;
                }
                // if this is the first new_screen received, set the initialized bit
                {
                    if !*self_for_listener.initialized.lock().unwrap() {
                        *self_for_listener.initialized.lock().unwrap() = true;
                    }
                }
            }
        });
    }

    pub fn render(
        &self,
        transform: Matrix2d,
        ctx: &Context,
        gl: &mut GlGraphics,
        assets: &mut Assets,
    ) {
        if !*self.initialized.lock().unwrap() {
            return;
        }
        {
            let mut screen = self.screen.lock().unwrap();
            screen.render(transform, ctx, gl, assets);
        }
        once!("render was done");
    }

    pub fn get_lines_completed(&mut self) -> u64 {
        {
            let mut screen = self.screen.lock().unwrap();
            let lines = screen.new_completed_lines;
            screen.new_completed_lines = 0;
            return lines;
        }
    }
}
