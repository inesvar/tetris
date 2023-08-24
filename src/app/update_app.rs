//! Defines the update function of [App].
//!
//! [update()](App::update()) is called before each render when the game is active.
use super::{App, PlayerConfig, RunningState, ViewState};
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
            self.widget_manager
                .update_settings(&mut self.keybindings_manager);
        } else if self.running == RunningState::Running {
            // on ne fait pas d'update quand running == false
            self.clock += args.dt;
            self.frame_counter = self.frame_counter.wrapping_add(1);
            if let PlayerConfig::TwoRemote = self.player_config {
                for player in &mut self.local_players {
                    let completed_lines = self.remote_players[0].get_lines_completed();
                    if completed_lines != 0 {
                        println!("the adversary completed {} lines", completed_lines);
                        player.add_garbage(completed_lines);
                    }
                }
            }
            for player in &mut self.local_players {
                player.update(
                    &self.keybindings_manager,
                    self.frame_counter,
                    fall_speed_divide,
                    freeze,
                );
            }
        }
        // taking into account the player states after a new piece was added
        // two options :
        // either the player didn't lose => nothing to do
        // there was a game over => the running must be set to NotRunning
        for player in &self.local_players {
            if player.get_game_over() == true {
                self.running = RunningState::NotRunning;
            }
        }
        /* for player in &self.remote_players {
            if player.get_game_over() == true {
                self.running = RunningState::NotRunning;
            }
        } */
        // then eventually change the view
        let result = self.widget_manager.update_view();
        match result {
            ButtonType::ToPause => self.pause(),
            ButtonType::Nothing => {}
            ButtonType::BackToMainMenu => {
                if self.view_state == ViewState::SinglePlayerGame
                    && self.running == RunningState::Running
                {
                    self.pause()
                };
                self.set_view(ViewState::MainMenu)
            }
            ButtonType::ToSettings => self.set_view(ViewState::Settings),
            ButtonType::ToSinglePlayerGame => {
                if self.view_state == ViewState::MainMenu && self.running == RunningState::Paused {
                    self.pause()
                };
                self.set_view(ViewState::SinglePlayerGame)
            }
            _ => {}
        }
    }
}
