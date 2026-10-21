/// View state indicates what is on screen.
/// The game states are handled differently with help of the [ViewState::is_game()] method.
#[derive(Debug, PartialEq, Default)]
pub enum ViewState {
    #[default]
    MainMenu,
    Settings,
    JoinRoom,
    CreateRoom,
    Local,    // TODO: this is the same view, conceptually it's the Game View
    TwoLocal, // TODO: this is the same view, conceptually it's the Game View
    Remote,   // TODO: this is the same view, conceptually it's the Game View
}

impl ViewState {
    pub(super) fn is_game(&self) -> bool {
        matches!(self, Self::Local | Self::TwoLocal | Self::Remote)
    }
}