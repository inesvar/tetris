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
    /// - update the data in the view, for instance :
    ///     - in [ViewState::Settings], update the settings
    ///     - if the game is running, update the grid and check that the game still runs
    /// - change the view if necessary
    pub fn update(&mut self, args: &UpdateArgs) {
        // TODO: split in two functions
        // first apply the changes inside the views
        if self.view_state == ViewState::Settings {
            for (id, widget_manager) in self.widget_manager.iter_mut().enumerate() {
                widget_manager.update_settings(&mut self.keybindings_manager[id]);
            }
        } else if self.view_state == ViewState::CreateRoom {
            self.widget_manager[0].update_clipboard();
        } else if self.view_state == ViewState::JoinRoom {
            self.widget_manager[0].update_clipboard();
            self.widget_manager[0].update_from_text();
        } else if self.view_state.is_game() && self.running == RunningState::Starting {
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
        } else if self.view_state.is_game() && self.running == RunningState::Running {
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
                    self.fall_speed_divide,
                    self.freeze,
                );
            }
            // taking into account the player states after a new piece was added
            // two options :
            // either the player didn't lose => nothing to do
            // there was a game over => the running must be set to NotRunning
            let mut game_over = false;
            for player in &self.local_players {
                if player.get_game_over() {
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

            // update the falling speed and freeze speed
            match self.clock {
                i if i <= 30.0 => {
                    self.fall_speed_divide = 50;
                    self.freeze = 50
                }
                i if i <= 60.0 => {
                    self.fall_speed_divide = 40;
                    self.freeze = 50
                }
                i if i <= 90.0 => {
                    self.fall_speed_divide = 30;
                    self.freeze = 50
                }
                i if i <= 120.0 => {
                    self.fall_speed_divide = 20;
                    self.freeze = 50
                }
                _ => {
                    self.fall_speed_divide = 15;
                    self.freeze = 50
                }
            }
        }

        // then eventually change the view
        let result = self.widget_manager[0].update_view();
        match result {
            ButtonType::ToPause => {
                if self.view_state.is_game() {
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
            ButtonType::ToSettings => {
                if self.running == RunningState::Running {
                    self.pause()
                };
                self.set_view(ViewState::Settings)
            }
            ButtonType::ToSinglePlayerGame => {
                if self.player_config != PlayerConfig::Local {
                    self.set_player_config(PlayerConfig::Local);
                }
                self.set_view(ViewState::Local)
            }
            ButtonType::ToCreateRoom => self.set_view(ViewState::CreateRoom),
            ButtonType::ToJoinRoom => {
                if self.player_config == PlayerConfig::Local {
                    self.set_view(ViewState::JoinRoom)
                } else if self.player_config.is_remote() {
                    self.set_view(ViewState::Remote)
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
                self.set_view(ViewState::Remote);
                self.send_message(MessageType::Hello(local_ip));
                self.local_players[0].send_serialized();
            }
            ButtonType::ToTwoLocalGame => {
                if self.player_config != PlayerConfig::TwoLocal {
                    self.set_player_config(PlayerConfig::TwoLocal);
                }
                self.set_view(ViewState::TwoLocal);
            }
            ButtonType::BackToGame => match &self.player_config {
                PlayerConfig::TwoLocal => self.set_view(ViewState::TwoLocal),
                PlayerConfig::Local => self.set_view(ViewState::Local),
                PlayerConfig::TwoRemote {
                    local_ip: _,
                    remote_ip: _,
                } => self.set_view(ViewState::Remote),
                _ => unreachable!(),
            },
            _ => {}
        }
    }
}
