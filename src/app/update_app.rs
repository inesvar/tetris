//! Defines the update function of [App].
//!
//! [update()](App::update()) is called before each render when the game is active.
use super::{remote::MessageType, App, Countdown, PlayerConfig, RunningState, ViewState};
use crate::ui::interactive_widget_manager::ButtonType;
use piston::UpdateArgs;

impl App<'_> {
    /// update is called before each render so that the informations on the screen are as recent as possible.
    ///
    /// It's responsible for the following :
    /// - updating the state of the view :
    ///     - in [ViewState::Settings], updating the settings
    ///     - if the game is running, updating the local players and checking that the game still runs
    /// - updating the view through the widget_manager
    ///
    pub fn update(&mut self, args: &UpdateArgs, fall_speed_divide: u64, freeze: u64) {
        // first apply the changes inside the views
        if self.view_state == ViewState::Settings {
            for (id, widget_manager) in self.widget_manager.iter_mut().enumerate() {
                widget_manager
                    .update_settings(&mut self.keybindings_manager[id]);
            }
        } else if self.view_state == ViewState::CreateRoom {
            self.widget_manager[0].update_clipboard();
        } else if self.view_state == ViewState::JoinRoom {
            self.widget_manager[0].update_clipboard();
            self.widget_manager[0].update_from_text();
        } else if self.view_state == ViewState::Game && self.running == RunningState::Starting {
            self.clock += args.dt;
            match self.clock {
                i if i < 1.0 => self.countdown(&Countdown::Three),
                i if i < 2.0 => self.countdown(&Countdown::Two),
                i if i < 3.0 => self.countdown(&Countdown::One),
                _ => self.start(),
            }
            for player in &mut self.local_players {
                player.send_serialized();
            }
        } else if self.view_state == ViewState::Game && self.running == RunningState::Running {
            self.clock += args.dt;
            self.frame_counter = self.frame_counter.wrapping_add(1);
            if let PlayerConfig::TwoRemote {
                local_ip: _,
                remote_ip: _,
            } = self.player_config
            {
                // add garbage
                for player in &mut self.local_players {
                    let completed_lines = self.remote_player[0].get_lines_completed();
                    if completed_lines != 0 {
                        println!("the adversary completed {} lines", completed_lines);
                        player.add_garbage(completed_lines);
                    }
                }
            } else if let PlayerConfig::TwoLocal = self.player_config {
                // add garbage
                let completed_lines = self.local_players[0].get_lines_completed();
                if completed_lines != 0 {
                    self.local_players[1].add_garbage(completed_lines);
                }
                let completed_lines = self.local_players[1].get_lines_completed();
                if completed_lines != 0 {
                    self.local_players[0].add_garbage(completed_lines);
                }
            }
            // update
            for (id, player) in self.local_players.iter_mut().enumerate() {
                player.update(
                    &self.keybindings_manager[id],
                    self.frame_counter,
                    fall_speed_divide,
                    freeze,
                );
            }
            // taking into account the player states after a new piece was added
            // two options :
            // either the player didn't lose => nothing to do
            // there was a game over => the running must be set to NotRunning
            let mut game_over = false;
            for player in &self.local_players {
                if player.get_game_over() == true {
                    game_over = true;
                }
            }

            /* // same for remote players
            for player in &self.remote_player {
                if player.get_game_flow() == GameFlowChange::GameOver {
                    game_over = true;
                }
            } */

            if game_over {
                self.game_over();
            }
        }

        // then eventually change the view
        let result = self.widget_manager[0].update_view();
        match result {
            ButtonType::ToPause => {
                if self.view_state == ViewState::Game {
                    self.pause()
                }
            }
            ButtonType::Nothing => {}
            ButtonType::BackToMainMenu => {
                println!("back to main menu");
                if self.running == RunningState::Running {
                    self.pause()
                };
                self.set_view(ViewState::MainMenu)
            }
            ButtonType::ToSettings => self.set_view(ViewState::Settings),
            ButtonType::ToSinglePlayerGame => {
                self.set_player_config(PlayerConfig::Local);
                self.set_view(ViewState::Game)
            }
            ButtonType::ToCreateRoom => self.set_view(ViewState::CreateRoom),
            ButtonType::ToJoinRoom => {
                if self.player_config == PlayerConfig::Local {
                    self.set_view(ViewState::JoinRoom)
                } else if self.player_config.is_remote() {
                    self.set_view(ViewState::Game)
                }
            }
            ButtonType::ToTwoRemoteGameInfo {
                local_ip,
                remote_ip,
            } => {
                self.set_player_config(PlayerConfig::TwoRemote {
                    local_ip: local_ip.clone(),
                    remote_ip,
                });
                self.set_view(ViewState::Game);
                self.send_message(MessageType::HelloMsg(local_ip));
            }
            _ => {}
        }
    }
}
