//! Define the update function of [App].
//!
//! [update()](App::update()) is called before each render when the game is active.
use super::{remote::OutboundMessage, App, PlayerConfig, RunningState, ViewState};
use crate::utils::formattings::format_seconds;
use piston::UpdateArgs;
use ui_tetris::interactive_widget_manager::{ButtonType, TextType};

impl App {
    fn update_running_game(&mut self) {
        self.widget_manager
            .set_text(TextType::Timer, format_seconds(self.game.clock));
        self.frame_counter = self.frame_counter.wrapping_add(1);
        // update the falling speed and freeze speed
        match self.game.clock {
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

        if let PlayerConfig::TwoRemote {
            local_ip: _,
            remote_ip: _,
        } = self.game.player_config
        {
            // add garbage
            for player in &mut self.game.local_players {
                let completed_lines = self.game.remote_player[0].get_garbage_to_send();
                if completed_lines != 0 {
                    println!("the adversary completed {} lines", completed_lines);
                }
                player.push_garbage(completed_lines);
            }
        }
        // update
        let mut garbage_to_send = Vec::new();
        for local_player in &mut self.game.local_players {
            let res = local_player.update(self.frame_counter, self.fall_speed_divide, self.freeze);
            let Ok(garbage) = res else {
                self.game.game_over();
                return;
            };
            garbage_to_send.push(garbage);
        }

        if let PlayerConfig::TwoLocal = self.game.player_config {
            // add garbage
            self.game.local_players[1].push_garbage(garbage_to_send[0]);
            self.game.local_players[0].push_garbage(garbage_to_send[1]);
        } else if let PlayerConfig::TwoRemote {
            local_ip: _,
            remote_ip: _,
        } = self.game.player_config
        {
            self.game.send_message(OutboundMessage::TetrisPlayer((
                self.game.local_players[0].player_data(),
                garbage_to_send[0],
            )));
        }
    }
}

impl App {
    /// update is called before each render so that the informations on the screen are as recent as possible.
    ///
    /// It's responsible for the following :
    /// - update the data in the view, for instance :
    ///     - in [ViewState::Settings], update the settings
    ///     - if the game is running, update the grid and check that the game still runs
    /// - change the view if necessary
    pub fn update(&mut self, args: &UpdateArgs) {
        self.game.clock += args.dt;
        if self.view_state == ViewState::CreateRoom {
            self.widget_manager.update_clipboard();
        } else if self.view_state == ViewState::JoinRoom {
            self.widget_manager.update_clipboard();
            self.widget_manager.update_from_text();
        } else if self.view_state.is_game() && self.game.running == RunningState::Starting {
            self.widget_manager
                .set_text(TextType::Timer, format_seconds(0.0));
            if self.game.clock > 3.0 {
                self.game.start();
            }
            for player in &self.game.local_players {
                self.game
                    .send_message(OutboundMessage::TetrisPlayer((player.player_data(), 0)));
            }
        } else if self.view_state.is_game() && self.game.running == RunningState::Running {
            self.update_running_game();
        }

        // then eventually change the view
        let result = self.widget_manager.update_view();
        match result {
            ButtonType::ToPause => {
                if self.view_state.is_game() {
                    self.game.pause()
                }
            }
            ButtonType::Nothing => {}
            ButtonType::BackToMainMenu => {
                println!("back to main menu");
                if self.game.running == RunningState::Running {
                    self.game.pause()
                };
                self.set_view(ViewState::MainMenu)
            }
            ButtonType::ToSettings => {
                if self.game.running == RunningState::Running {
                    self.game.pause()
                };
                self.set_view(ViewState::Settings)
            }
            ButtonType::ToSinglePlayerGame => {
                if self.game.player_config != PlayerConfig::Local {
                    self.game.set_player_config(PlayerConfig::Local);
                }
                self.set_view(ViewState::Local)
            }
            ButtonType::ToCreateRoom => self.set_view(ViewState::CreateRoom),
            ButtonType::ToJoinRoom => {
                if self.game.player_config == PlayerConfig::Local {
                    self.set_view(ViewState::JoinRoom)
                } else if self.game.player_config.is_remote() {
                    self.set_view(ViewState::Remote)
                }
            }
            ButtonType::ToTwoRemoteGameInfo {
                local_ip,
                remote_ip,
            } => {
                self.game.set_player_config(PlayerConfig::TwoRemote {
                    local_ip: local_ip.clone(),
                    remote_ip,
                });
                self.set_view(ViewState::Remote);
                self.game.send_message(OutboundMessage::Hello(local_ip));
                self.game.send_message(OutboundMessage::TetrisPlayer((
                    self.game.local_players[0].player_data(),
                    0,
                )));
            }
            ButtonType::ToTwoLocalGame => {
                if self.game.player_config != PlayerConfig::TwoLocal {
                    self.game.set_player_config(PlayerConfig::TwoLocal);
                }
                self.set_view(ViewState::TwoLocal);
            }
            ButtonType::BackToGame => match &self.game.player_config {
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
