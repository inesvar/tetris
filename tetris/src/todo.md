- CLEAN split TetrisGame into multiple structs
- THINK should the text rendering from ui_tetris go to render_tetris ?
- BUG if one key binds to multiple actions 
- PERF use tokio to outsource tcp comms
- FEAT today key inputs are sent to the tetris game and mouse inputs to the app... something more flexible would be great
- BUG the tetrominos should not change the first time the game is started...
- CLEAN ViewState::Game should not contain a number
- THINK understand the state machine of death and find a more readable way to write it
- CLEAN handle all kinds of errors that aren't handled

# new features
- CHECK fix paths for Windows in assets.rs -> done, has to be checked
- FEAT press key s open settings (requires some planning before, the code is messy)
- USER output an error when playing remote doesn't work