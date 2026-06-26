#[derive(PartialEq, Debug, Default)]
pub enum PlayerConfig {
    #[default]
    Local,
    TwoLocal,
    TwoRemote { local_ip: String, remote_ip: String },
    Viewer(String),
}

impl PlayerConfig {
    pub fn is_remote(&self) -> bool {
        matches!(
            self,
            PlayerConfig::TwoRemote {
                local_ip: _,
                remote_ip: _,
            } | PlayerConfig::Viewer(_)
        )
    }

    pub fn is_multiplayer(&self) -> bool {
        matches!(
            self,
            PlayerConfig::TwoRemote {
                local_ip: _,
                remote_ip: _,
            } | PlayerConfig::TwoLocal
        )
    }
}
