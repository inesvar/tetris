use crate::app::RunningState;
use crate::assets::Assets;
use crate::circular_buffer::CircularBuffer;
use crate::keyboard::Keyboard;
use crate::player_screen::PlayerScreen;
use crate::tetris_back_end::{
    new_tetromino_bag, TetrisGrid, Tetromino, TetrominoKind, TranslationRotation,
};
use crate::{once, settings::*};
use graphics::types::Matrix2d;
use opengl_graphics::GlGraphics;
use piston::Key;
use piston_window::Context;
use rand::SeedableRng;
use rand_pcg::Pcg32;
use serde::{Deserialize, Serialize};
use std::net::TcpStream;

#[derive(Serialize, Deserialize)]
pub struct LocalPlayer {
    /// player_screen contains all attributes visible on the screen
    /// like the TetrisGrid, the active Tetromino, the file of next Tetromino, etc.
    ///
    /// PlayerScreen, not LocalPlayer, is rendered.
    /// PlayerScreen, not LocalPlayer, will be sent to the remote.
    player_screen: PlayerScreen,
    keyboard: Keyboard,
    /// freeze_frame indicates when to freeze the active_tetromino and get a new one.
    ///
    /// freeze_frame is updated when a tetromino reaches the bottom of the grid.
    freeze_frame: u64,
    bag_of_tetromino: Vec<TetrominoKind>,
    sender: bool,
    garbage_to_be_added: u64,
    #[serde(skip, default = "new_pcg")]
    rng: Pcg32,
}

fn new_pcg() -> Pcg32 {
    Pcg32::seed_from_u64(0)
}

impl LocalPlayer {
    pub fn new(seed: u64, sender: bool) -> Self {
        let grid = TetrisGrid::new(DEFAULT_GRID_X, DEFAULT_GRID_Y, NB_COLUMNS, NB_ROWS);
        let mut rng = Pcg32::seed_from_u64(seed);
        let mut bag_of_tetromino = new_tetromino_bag(BAG_SIZE, &mut rng);
        let first_tetromino =
            Tetromino::new(bag_of_tetromino.pop().unwrap(), &grid.matrix[..]).unwrap();
        let mut fifo_next_tetromino = CircularBuffer::<NB_NEXT_TETROMINO, Tetromino>::new();
        for _ in 0..NB_NEXT_TETROMINO {
            if let Some(t) = bag_of_tetromino.pop() {
                fifo_next_tetromino.push(Tetromino::new_unchecked(t));
            } else {
                bag_of_tetromino = new_tetromino_bag(BAG_SIZE, &mut rng);
                if let Some(t) = bag_of_tetromino.pop() {
                    fifo_next_tetromino.push(Tetromino::new_unchecked(t));
                } else {
                    unreachable!();
                }
            }
        }

        let player_screen = PlayerScreen {
            grid,
            score: 0,
            game_over: false,
            new_completed_lines: 0,
            active_tetromino: first_tetromino,
            saved_tetromino: None,
            fifo_next_tetromino,
            ghost_tetromino: None,
            serialize_as_msg: true.into(),
        };

        LocalPlayer {
            player_screen,
            keyboard: Keyboard::new(),
            freeze_frame: 0, // that's about 10 billion years at 60fps
            bag_of_tetromino,
            sender,
            garbage_to_be_added: 0,
            rng,
        }
    }

    /// Sets a new active_tetromino when the precedent one is frozen.
    fn get_new_tetromino(&mut self) {
        // Refill the bag if necessary
        if self.bag_of_tetromino.is_empty() {
            self.bag_of_tetromino = new_tetromino_bag(BAG_SIZE, &mut self.rng);
        }
        // Check if there's enough place on the grid for a new tetromino
        let possible_active = self.player_screen.fifo_next_tetromino.pop().unwrap();
        if possible_active
            .check_possible(&self.player_screen.grid.matrix, TranslationRotation::null())
            .is_err()
        {
            // If not, it's a lock out situation
            // set the game_over flag and return the tetromino to the bag
            self.declare_game_over();
            self.player_screen
                .fifo_next_tetromino
                .push_front(possible_active);
            return;
        }
        // Add a new tetromino to the file to replace the one that was taken
        self.player_screen
            .fifo_next_tetromino
            .push(Tetromino::new_unchecked(
                self.bag_of_tetromino.pop().unwrap(),
            ));
        self.player_screen.active_tetromino = possible_active;
    }

    pub fn add_garbage(&mut self, completed_lines: u64) {
        self.garbage_to_be_added = completed_lines;
    }

    pub fn send_serialized(&self) {
        if let Ok(stream) = TcpStream::connect(VIEWER_IP) {
            serde_cbor::to_writer::<TcpStream, PlayerScreen>(stream, &self.player_screen).unwrap();
        }
        once!("sent serialized data to {}", VIEWER_IP);
    }
}

impl LocalPlayer {
    pub fn restart(&mut self) {
        self.player_screen.game_over = false;
        self.player_screen.score = 0;
        self.player_screen.grid.null();
    }

    pub fn get_game_over(&self) -> bool {
        self.player_screen.game_over
    }

    pub fn declare_game_over(&mut self) {
        self.player_screen.game_over = true;
        self.player_screen.saved_tetromino = None;
    }

    pub fn render(
        &mut self,
        transform: Matrix2d,
        ctx: &Context,
        gl: &mut GlGraphics,
        assets: &mut Assets,
    ) {
        self.player_screen.render(transform, ctx, gl, assets);
    }

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
    ///
    /// - sending the serialized data to the remote
    ///     // TODO move the sending somewhere else?
    /// -
    ///
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
            if self.keyboard.is_any_pressed(&keybindings.fall_keys) {
                if self
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
            } else if self.keyboard.is_any_delay_pressed(&keybindings.left_keys) {
                self.player_screen
                    .active_tetromino
                    .left(&self.player_screen.grid.matrix);
            } else if self.keyboard.is_any_delay_pressed(&keybindings.right_keys) {
                self.player_screen
                    .active_tetromino
                    .right(&self.player_screen.grid.matrix);
            }
        }

        /**********************************
         *    EVERY FALL_SPEED_DIVIDE     *
         *              ---               *
         *  "continuous" slower actions   *
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

        // Set the number of completed lines to 0
        if self.player_screen.new_completed_lines != 0 {
            println!(
                "the {} completed lines were sent to the adversary and they were reset to 0",
                self.player_screen.new_completed_lines
            );
            self.player_screen.new_completed_lines = 0;
        }
    }

    /// handle_key_press is called when a key is pressed.
    ///
    /// It moves the tetromino accordingly if needed, it's responsible for all tetromino events except for the following (which are handled in update) :
    /// - the tetromino "falling" down naturally
    /// - the tetromino "freezing" at the bottom
    /// - the tetromino moving continuously to the right (resp. left) on a long key press
    ///
    /// Its also responsible for the events :
    /// - pause
    /// - restart
    pub fn handle_key_press(
        &mut self,
        keybindings: &Keybindings,
        key: Key,
        running: RunningState,
    ) -> KeyPress {
        self.keyboard.set_pressed(key);

        // the unactive game only listens to the RESTART_KEYS
        if running == RunningState::NotRunning && !self.keyboard.is_any_pressed(&RESTART_KEYS) {
            return KeyPress::Other;
        } else if running == RunningState::NotRunning && self.keyboard.is_any_pressed(&RESTART_KEYS)
        {
            return KeyPress::Restart;
        }

        // the paused game only listens to the PAUSE_KEYS
        if running == RunningState::Paused && !self.keyboard.is_any_pressed(&PAUSE_KEYS) {
            return KeyPress::Other;
        } else if running == RunningState::Paused && self.keyboard.is_any_pressed(&PAUSE_KEYS) {
            return KeyPress::Resume;
            // the game pauses if PAUSE_KEYS are pressed
        } else if running == RunningState::Running && self.keyboard.is_any_pressed(&PAUSE_KEYS) {
            return KeyPress::Pause;
        }

        /******************************
         *         ACTIVE GAME        *
         ******************************/

        // Pressed once events
        if self
            .keyboard
            .is_any_pressed(&keybindings.rotate_clockwise_keys)
        {
            // rotate once the tetromino
            self.player_screen
                .active_tetromino
                .turn_clockwise(&self.player_screen.grid.matrix);
        } else if self
            .keyboard
            .is_any_pressed(&keybindings.rotate_counterclockwise_keys)
        {
            // rotate once the tetromino
            self.player_screen
                .active_tetromino
                .turn_counterclockwise(&self.player_screen.grid.matrix);
        }

        if self
            .keyboard
            .is_any_pressed(&keybindings.hold_tetromino_keys)
        {
            // hold the tetromino
            if let Some(mut saved) = self.player_screen.saved_tetromino {
                self.player_screen.active_tetromino.reset_position();

                std::mem::swap(&mut saved, &mut self.player_screen.active_tetromino);
                self.player_screen.saved_tetromino = Some(saved);
            } else {
                self.player_screen.active_tetromino.reset_position();

                self.player_screen.saved_tetromino = Some(self.player_screen.active_tetromino);
                self.get_new_tetromino();
            }
        }

        // move the tetromino left or right
        if self.keyboard.is_any_pressed(&keybindings.left_keys) {
            self.player_screen
                .active_tetromino
                .left(&self.player_screen.grid.matrix);
        } else if self.keyboard.is_any_pressed(&keybindings.right_keys) {
            self.player_screen
                .active_tetromino
                .right(&self.player_screen.grid.matrix);
        }

        if self.keyboard.is_any_pressed(&keybindings.hard_drop_keys) {
            // hard drop the tetromino
            self.player_screen
                .active_tetromino
                .hard_drop(&self.player_screen.grid.matrix);
            match self
                .player_screen
                .grid
                .freeze_tetromino(&mut self.player_screen.active_tetromino)
            {
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
                None => self.declare_game_over(),
            }
        }
        KeyPress::Other
    }

    pub fn handle_key_release(&mut self, key: Key) {
        self.keyboard.set_released(key);
    }
}

pub enum KeyPress {
    Restart,
    Resume,
    Pause,
    Other,
}
