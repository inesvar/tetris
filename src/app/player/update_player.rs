//! Defines the update function of [LocalPlayer].
//!
//! [update()](LocalPlayer::update()) is called before each render when the game is active.
use super::{back_end::TranslationRotation, LocalPlayer};
use crate::settings::Keybindings;

impl LocalPlayer {
    /// update is called before each render so that the informations on the screen are as recent as possible.
    ///
    /// It's responsible for the following tetromino events :
    /// - the tetromino "falling" down naturally
    /// - the tetromino freezing at the bottom and a new one appearing at the top
    /// - the tetromino moving continuously to the right (resp. left) on a long key press
    ///
    /// It's also responsible for :
    /// - updating the keyboard clock
    /// - updating the ghost tetromino
    /// - adding garbage
    /// - sending the serialized data to the remote
    ///
    /// When the game is paused or inactive, update should not be called.
    pub fn update(
        &mut self,
        keybindings: &Keybindings,
        frame_counter: u64,
        fall_speed_divide: u64,
        freeze: u64,
    ) {
        /* Actions in this function have to be carefully ordered so that there are no uncoherences.
         *
         * For instance, garbage has to be added AFTER the tetromino is moved because it hasn't been rendered yet
         * so the player couldn't adapt.
         */

        /**********************************
         *   MOVING the ACTIVE_TETROMINO  *
         **********************************/

        /**********************************
         *         EVERY 5 TICKS          *
         *              ---               *
         *     "continuous" actions       *
         **********************************/

        // Translate the tetromino down on a key press
        if frame_counter % 5 == 0 {
            if self.keyboard.is_any_delay_pressed(&keybindings.fall_keys)
                && self
                    .player_screen
                    .active_tetromino
                    .fall(&self.player_screen.grid.matrix)
                    .is_err()
                && self.freeze_frame < frame_counter
            {
                // if the tetromino reaches the bottom, set the freeze_frame
                self.freeze_frame = frame_counter + freeze;
            }
            // Translate the tetromino right or left on a long key press
            if self.keyboard.is_any_delay_pressed(&keybindings.left_keys)
                && !self.keyboard.is_any_delay_pressed(&keybindings.right_keys)
            {
                self.player_screen
                    .active_tetromino
                    .left(&self.player_screen.grid.matrix);
            }
            if self.keyboard.is_any_delay_pressed(&keybindings.right_keys)
                && !self.keyboard.is_any_delay_pressed(&keybindings.left_keys)
            {
                self.player_screen
                    .active_tetromino
                    .right(&self.player_screen.grid.matrix);
            }
        }

        /**********************************
         *    EVERY FALL_SPEED_DIVIDE     *
         *              ---               *
         *  "continuous" slower actions   *
         *       periodic actions         *
         **********************************/

        // move the tetromino down to emulate its fall
        if frame_counter % fall_speed_divide == 0
            && self
                .player_screen
                .active_tetromino
                .fall(&self.player_screen.grid.matrix)
                .is_err()
            && self.freeze_frame < frame_counter
        {
            // if the tetromino reaches the bottom, set the freeze_frame
            self.freeze_frame = frame_counter + freeze;
        }

        /**********************************
         *        AT FREEZE_FRAME         *
         *              ---               *
         *       only occasionally        *
         **********************************/

        // Freeze the tetromino if it reached the bottom previously and can't go down anymore
        if frame_counter == self.freeze_frame
            && self
                .player_screen
                .active_tetromino
                .check_possible(&self.player_screen.grid.matrix, TranslationRotation::fall())
                .is_err()
        {
            match self
                .player_screen
                .grid
                .freeze_tetromino(&mut self.player_screen.active_tetromino)
            {
                // if lines were clearing by freezing the tetromino, set the attribute new_completed_lines
                Some(completed_lines) => {
                    self.player_screen.new_completed_lines = completed_lines;
                    if self.player_screen.new_completed_lines != 0 {
                        println!(
                            "{} lines were completed",
                            self.player_screen.new_completed_lines
                        );
                    }
                    self.player_screen.score += self.player_screen.new_completed_lines;
                    self.get_new_tetromino();
                }
                // if the tetromino froze above the visible grid, it's game over !
                None => self.declare_game_over(),
            }
        }

        /**********************************
         *          AT EVERY TICK         *
         *              ---               *
         *      preparing the new render  *
         **********************************/

        // Updates the time for the keyboard
        self.keyboard.update();

        // Updates the ghost_tetromino
        let mut ghost = self.player_screen.active_tetromino.make_ghost_copy();
        ghost.hard_drop(&self.player_screen.grid.matrix);
        self.player_screen.ghost_tetromino = Some(ghost);

        // Adds garbage to the grid
        self.player_screen
            .grid
            .add_garbage(self.garbage_to_be_added);
        self.garbage_to_be_added = 0;

        // Send the player_screen data if necessary
        if self.sender {
            self.send_serialized();
        }
    }
}
