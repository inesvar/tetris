use super::MessageType;
use crate::{
    app::{GameFlowChange, PlayerScreen},
    assets::Assets,
    once,
};
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
    game_flow_message: Arc<Mutex<GameFlowChange>>,
}

impl RemotePlayer {
    pub fn new() -> Self {
        let arc = Arc::new(Mutex::new(PlayerScreen::empty()));
        RemotePlayer {
            screen: arc,
            first_screen_received: Arc::new(Mutex::new(false)),
            game_flow_message: Arc::new(Mutex::new(GameFlowChange::GameOver)),
        }
    }

    pub fn listen(&self, local_ip: &str) {
        // building a second RemotePlayer that points to the same pointees than self
        // this is necessary because self can't be moved out to another thread
        let screen = Arc::clone(&self.screen);
        let first_screen_received = Arc::clone(&self.first_screen_received);
        let game_flow_message = Arc::clone(&self.game_flow_message);
        let self_for_listener = RemotePlayer {
            screen,
            first_screen_received,
            game_flow_message,
        };
        // creating a listener in a separate thread
        let listener = TcpListener::bind(local_ip).unwrap();
        thread::spawn(move || {
            // for each incoming message
            for stream in listener.incoming() {
                let stream = stream.unwrap();
                let message = serde_cbor::from_reader::<MessageType, TcpStream>(stream).unwrap();
                once!("unwrapped from packet from remote");
                match message {
                    MessageType::PlayerScreenMsg(new_screen) => {
                        self_for_listener.update_screen(new_screen)
                    }
                    MessageType::SettingsMsg(new_settings) => {
                        self_for_listener.update_game_flow(GameFlowChange::Sync(new_settings));
                    }
                    MessageType::GameOverMsg => {
                        self_for_listener.update_game_flow(GameFlowChange::GameOver);
                    }
                    MessageType::PauseMsg => {
                        self_for_listener.update_game_flow(GameFlowChange::Pause);
                    }
                    MessageType::RestartMsg => {
                        self_for_listener.update_game_flow(GameFlowChange::Restart);
                    }
                    MessageType::ResumeMsg => {
                        self_for_listener.update_game_flow(GameFlowChange::Resume);
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

    /// Updates the remote player with the game flow new message received.
    fn update_game_flow(&self, new_game_flow: GameFlowChange) {
        {
            let mut game_flow = self.game_flow_message.lock().unwrap();
            println!("game flow change to {:?}", new_game_flow);
            *game_flow = new_game_flow;
        }
    }

    /// Returns the game_flow_message and resets it
    pub(in crate::app) fn get_game_flow(&self) -> GameFlowChange {
        let mut last_game_flow: GameFlowChange = GameFlowChange::Other;
        {
            std::mem::swap(
                &mut last_game_flow,
                &mut self.game_flow_message.lock().unwrap(),
            );
            if last_game_flow != GameFlowChange::Other {
                println!("{last_game_flow:?} was read");
            }
        }
        return last_game_flow;
    }
}
