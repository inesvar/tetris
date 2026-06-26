- CLEAN the clock should pause during a game pause :
    the renderer and the app should have a separate clock
    the app should pause its clock during tetris pause, while the renderer should not
    but both should start when a new game is launched (countdown)
- PERF use tokio to outsource tcp comms
- FEAT today key inputs are sent to the tetris game and mouse inputs to the app... something more flexible would be great
- BUG the tetrominos should not change the first time the game is started...
- ~~PERF share one TcpStream~~
- CLEAN split and reorganize app.rs
- CLEAN understand the state machine of death
- CLEAN handle all kinds of errors that aren't handled

# new features
- CHECK fix paths for Windows in assets.rs -> done, has to be checked
- FEAT press key s open settings (requires some planning before, the code is messy)
- USER output an error when playing remote doesn't work