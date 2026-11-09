/// View state indicates what is on screen.
/// The game states are handled differently with help of the [ViewState::is_game()] method.
#[derive(Debug, PartialEq, Default)]
pub enum ViewState {
    #[default]
    MainMenu,
    Settings,
    JoinRoom,
    CreateRoom,
    Game(usize),
}
