pub(super) struct ScoreManager {
    back_to_back: bool,
}

pub(super) trait ScoredAction {
    fn score(&self) -> u32;
}
