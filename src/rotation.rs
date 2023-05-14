use serde::Serialize;

#[derive(Clone, Copy, Serialize)]
pub enum Rotation {
    R0,
    R1,
    R2,
    R3,
}

impl Rotation {
    pub fn clockwise(&mut self) {
        match self {
            Rotation::R0 => {
                *self = Rotation::R1;
            }
            Rotation::R1 => {
                *self = Rotation::R2;
            }
            Rotation::R2 => {
                *self = Rotation::R3;
            }
            Rotation::R3 => {
                *self = Rotation::R0;
            }
        }
    }

    pub fn counterclockwise(&mut self) {
        match self {
            Rotation::R0 => {
                *self = Rotation::R3;
            }
            Rotation::R1 => {
                *self = Rotation::R0;
            }
            Rotation::R2 => {
                *self = Rotation::R1;
            }
            Rotation::R3 => {
                *self = Rotation::R2;
            }
        }
    }
}
