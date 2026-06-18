#[allow(missing_docs)]
#[derive(PartialEq, Eq, Debug, Clone, Copy, Default)]
/// Types of line clears (simple line clear or T-spin ? how many cleared lines ?).
pub enum LineClear {
    #[default]
    None,
    Single,
    Double,
    Triple,
    Tetris,
    MiniTSpin,
    MiniTSpinSingle,
    MiniTSpinDouble,
    TSpin,
    TSpinSingle,
    TSpinDouble,
    TSpinTriple,
}

#[derive(Default, PartialEq, Debug, Clone, Copy)]
pub(super) enum LineClearType {
    #[default]
    Regular,
    TSpin,
    MiniTSpin,
}

impl LineClear {
    fn get_type(&self) -> LineClearType {
        match self {
            Self::MiniTSpin | Self::MiniTSpinSingle | Self::MiniTSpinDouble => {
                LineClearType::MiniTSpin
            }
            Self::TSpin | Self::TSpinSingle | Self::TSpinDouble | Self::TSpinTriple => {
                LineClearType::TSpin
            }
            _ => LineClearType::Regular,
        }
    }

    pub(super) fn new(nb_lines_cleared: u32, line_clear: LineClearType) -> Self {
        match line_clear {
            LineClearType::Regular => match nb_lines_cleared {
                0 => Self::None,
                1 => Self::Single,
                2 => Self::Double,
                3 => Self::Triple,
                4 => Self::Tetris,
                _ => unreachable!("More than 4 lines can't be cleared by a single tetromino"),
            },
            LineClearType::TSpin => match nb_lines_cleared {
                0 => Self::TSpin,
                1 => Self::TSpinSingle,
                2 => Self::TSpinDouble,
                3 => Self::TSpinTriple,
                _ => unreachable!("More than 3 lines can't be cleared by a T-spin"),
            },
            LineClearType::MiniTSpin => match nb_lines_cleared {
                0 => Self::MiniTSpin,
                1 => Self::MiniTSpinSingle,
                2 => Self::MiniTSpinDouble,
                _ => unreachable!("More than 2 line can't be cleared by a mini T-spin"),
            },
        }
    }

    pub(super) fn nb_lines_cleared(&self) -> u32 {
        match self {
            Self::None | Self::MiniTSpin | Self::TSpin => 0,
            Self::Single | Self::MiniTSpinSingle | Self::TSpinSingle => 1,
            Self::Double | Self::MiniTSpinDouble | Self::TSpinDouble => 2,
            Self::Triple | Self::TSpinTriple => 3,
            Self::Tetris => 4,
        }
    }

    pub(super) fn score(&self) -> u32 {
        match self {
            Self::None => 0,
            Self::Single | Self::MiniTSpin => 100,
            Self::MiniTSpinSingle => 200,
            Self::Double => 300,
            Self::TSpin | Self::MiniTSpinDouble => 400,
            Self::Triple => 500,
            Self::Tetris | Self::TSpinSingle => 800,
            Self::TSpinDouble => 1200,
            Self::TSpinTriple => 1600,
        }
    }

    pub(super) fn continues_back_to_back_sequence(&self) -> bool {
        !matches!(self, Self::Single | Self::Double | Self::Triple)
    }

    pub(super) fn begins_back_to_back_sequence(&self) -> bool {
        !matches!(
            self,
            Self::Single | Self::Double | Self::Triple | Self::MiniTSpin | Self::TSpin
        )
    }

    pub(super) fn has_back_to_back_bonus(&self) -> bool {
        !matches!(
            self,
            Self::Single | Self::Double | Self::Triple | Self::MiniTSpin | Self::TSpin
        )
    }
}
