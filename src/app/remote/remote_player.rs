use super::MessageType;
use crate::{app::PlayerScreen, assets::Assets, once, settings::SERVER_IP};
use graphics::{math::Matrix2d, Context};
use opengl_graphics::GlGraphics;
use std::{
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread,
};

pub struct RemotePlayer {
    screen: Arc<Mutex<PlayerScreen>>,
    first_screen_received: Arc<Mutex<bool>>,
}

impl RemotePlayer {
    pub fn new() -> Self {
        let arc = Arc::new(Mutex::new(PlayerScreen::empty()));
        RemotePlayer {
            screen: arc,
            first_screen_received: Arc::new(Mutex::new(false)),
        }
    }

    pub fn listen(&self) {
        // building a second RemotePlayer that points to the same pointees than self
        // this is necessary because self can't be moved out to another thread
        let screen = Arc::clone(&self.screen);
        let first_screen_received = Arc::clone(&self.first_screen_received);
        let self_for_listener = RemotePlayer {
            screen,
            first_screen_received,
        };
        // creating a listener in a separate thread
        let listener = TcpListener::bind(SERVER_IP).unwrap();
        thread::spawn(move || {
            // for each incoming message
            for stream in listener.incoming() {
                let stream = stream.unwrap();
                let message = serde_cbor::from_reader::<MessageType, TcpStream>(stream).unwrap();
                once!("unwrapped from {}", SERVER_IP);
                match message {
                    MessageType::PlayerScreenMsg(new_screen) => {
                        self_for_listener.update_screen(new_screen)
                    }
                    _ => {}
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
        if !*self.first_screen_received.lock().unwrap() {
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

    /// Updates the remote player with the new_screen received.
    fn update_screen(&self, new_screen: PlayerScreen) {
        {
            let mut local_screen = self.screen.lock().unwrap();
            // if the new_completed_lines haven't been read yet, ensure it's not rewritten
            if local_screen.new_completed_lines != 0 {
                let a = local_screen.new_completed_lines;
                *local_screen = new_screen;
                local_screen.new_completed_lines = a;
            } else {
                *local_screen = new_screen;
            }
        }
        // if this is the first new_screen received, set the first_screen_received bit
        {
            if !*self.first_screen_received.lock().unwrap() {
                *self.first_screen_received.lock().unwrap() = true;
            }
        }
    }
}
