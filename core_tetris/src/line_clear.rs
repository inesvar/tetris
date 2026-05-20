#![allow(unused)]
use super::ScoredAction;

pub(super) enum LineClear {
    None,
    Single,
    Double,
    Triple,
    Tetris,
    MiniTSpin,
    MiniTSpinSingle,
    TSpin,
    TSpinSingle,
    TSpinDouble,
    TSpinTriple,
}

impl LineClear {
    pub(super) fn new(nb_lines_cleared: u32) -> Self {
        match nb_lines_cleared {
            0 => Self::None,
            1 => Self::Single,
            2 => Self::Double,
            3 => Self::Triple,
            4 => Self::Tetris,
            _ => panic!("More than 4 lines can't be cleared by a single tetromino"),
        }
    }

    pub(super) fn mini_t_spin(nb_lines_cleared: u32) -> Self {
        match nb_lines_cleared {
            0 => Self::MiniTSpin,
            1 => Self::MiniTSpinSingle,
            _ => panic!("More than 1 line can't be cleared by a mini T-spin"),
        }
    }

    pub(super) fn t_spin(nb_lines_cleared: u32) -> Self {
        match nb_lines_cleared {
            0 => Self::TSpin,
            1 => Self::TSpinSingle,
            2 => Self::TSpinDouble,
            3 => Self::TSpinTriple,
            _ => panic!("More than 3 lines can't be cleared by a T-spin"),
        }
    }

    pub(super) fn nb_lines_cleared(&self) -> u64 {
        match self {
            LineClear::None | LineClear::MiniTSpin | LineClear::TSpin => 0,
            LineClear::Single | LineClear::MiniTSpinSingle | LineClear::TSpinSingle => 1,
            LineClear::Double | LineClear::TSpinDouble => 2,
            LineClear::Triple | LineClear::TSpinTriple => 3,
            LineClear::Tetris => 4,
        }
    }
}

impl ScoredAction for LineClear {
    fn score(&self) -> u32 {
        match self {
            LineClear::None => 0,
            LineClear::Single | LineClear::MiniTSpin => 100,
            LineClear::MiniTSpinSingle => 200,
            LineClear::Double => 300,
            LineClear::TSpin => 400,
            LineClear::Triple => 500,
            LineClear::Tetris | LineClear::TSpinSingle => 800,
            LineClear::TSpinDouble => 1200,
            LineClear::TSpinTriple => 1600,
        }
    }
}
