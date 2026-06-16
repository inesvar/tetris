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
            LineClear::None | LineClear::MiniTSpin | LineClear::TSpin => 0,
            LineClear::Single | LineClear::MiniTSpinSingle | LineClear::TSpinSingle => 1,
            LineClear::Double | LineClear::MiniTSpinDouble | LineClear::TSpinDouble => 2,
            LineClear::Triple | LineClear::TSpinTriple => 3,
            LineClear::Tetris => 4,
        }
    }

    pub(super) fn score(&self) -> u32 {
        match self {
            LineClear::None => 0,
            LineClear::Single | LineClear::MiniTSpin => 100,
            LineClear::MiniTSpinSingle => 200,
            LineClear::Double => 300,
            LineClear::TSpin | LineClear::MiniTSpinDouble => 400,
            LineClear::Triple => 500,
            LineClear::Tetris | LineClear::TSpinSingle => 800,
            LineClear::TSpinDouble => 1200,
            LineClear::TSpinTriple => 1600,
        }
    }
}
