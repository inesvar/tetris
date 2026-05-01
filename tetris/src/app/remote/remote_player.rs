use super::InboundMessage;
use crate::{
    app::{GameFlowChange, TetrisPlayer},
    once,
    settings::BAG_TYPE,
};
use rand::SeedableRng;
use rand_pcg::Pcg32;
use std::{
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex, MutexGuard},
    thread,
};

pub struct RemotePlayer {
    screen: Arc<Mutex<TetrisPlayer>>,
    first_screen_received: Arc<Mutex<bool>>,
    game_flow_message: Arc<Mutex<GameFlowChange>>,
}

impl RemotePlayer {
    pub fn new() -> Self {
        let mut rng = Pcg32::seed_from_u64(0);
        let arc = Arc::new(Mutex::new(TetrisPlayer::default(&mut rng, BAG_TYPE)));
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
                let message = serde_cbor::from_reader::<InboundMessage, TcpStream>(stream).unwrap();
                once!("unwrapped from packet from remote");
                match message {
                    InboundMessage::TetrisPlayer(new_screen) => {
                        self_for_listener.update_screen(new_screen)
                    }
                    InboundMessage::Settings(new_settings) => {
                        self_for_listener.update_game_flow(GameFlowChange::Sync(new_settings));
                    }
                    InboundMessage::GameOver => {
                        self_for_listener.update_game_flow(GameFlowChange::GameOver);
                    }
                    InboundMessage::Pause => {
                        self_for_listener.update_game_flow(GameFlowChange::Pause);
                    }
                    InboundMessage::Restart => {
                        self_for_listener.update_game_flow(GameFlowChange::Restart);
                    }
                    InboundMessage::Resume => {
                        self_for_listener.update_game_flow(GameFlowChange::Resume);
                    }
                    InboundMessage::Hello(remote_ip) => {
                        self_for_listener.update_game_flow(GameFlowChange::Hello(remote_ip));
                    }
                    InboundMessage::Kill => {
                        break;
                    }
                }
            }
            println!("thread over");
        });
    }

    pub fn get_lines_completed(&mut self) -> u64 {
        {
            let mut screen = self.screen.lock().unwrap();
            let lines = screen.new_completed_lines();
            *screen.new_completed_lines_mut() = 0;
            lines
        }
    }

    /// Updates the remote player with the new_screen received.
    fn update_screen(&self, new_screen: TetrisPlayer) {
        {
            let mut local_screen = self.screen.lock().unwrap();
            // TODO this should be simpler
            // if the new_completed_lines haven't been read yet, ensure it's not rewritten
            if local_screen.new_completed_lines() != 0 {
                let a = local_screen.new_completed_lines();
                *local_screen = new_screen;
                *local_screen.new_completed_lines_mut() = a;
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
        let mut last_game_flow: GameFlowChange = GameFlowChange::NoChange;
        {
            std::mem::swap(
                &mut last_game_flow,
                &mut self.game_flow_message.lock().unwrap(),
            );
            if last_game_flow != GameFlowChange::NoChange {
                println!("{last_game_flow:?} was read");
            }
        }
        last_game_flow
    }

    pub(in crate::app) fn received_first_screen(&self) -> bool {
        *self.first_screen_received.lock().unwrap()
    }

    pub(in crate::app) fn get_player(&self) -> MutexGuard<'_, TetrisPlayer> {
        self.screen.lock().unwrap()
    }

    pub(in crate::app) fn score(&self) -> u64 {
        {
            let player = self.get_player();
            player.score()
        }
    }
}
