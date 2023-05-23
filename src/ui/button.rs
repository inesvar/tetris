use crate::assets::Assets;
use crate::graphics::Transformed;
use graphics::{color, rectangle, Context};
use opengl_graphics::GlGraphics;
use piston::RenderArgs;
use crate::ui::text::Text;

pub struct Button {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub text: Text,
    pub pressed: bool,
}

impl Button {
    pub fn new(x: f64, y: f64, width: f64, height: f64, text: String) -> Self {
        Button {
            x,
            y,
            width,
            height,
            text: Text::new(text, 16, 0.0, 0.0, color::RED),
            pressed: false,
        }
    }

    pub fn is_clicked(&self, x: f64, y: f64) -> bool {
        x >= self.x && x <= self.x + self.width && y >= self.y && y <= self.y + self.height
    }

    pub fn is_pressed(&self) -> bool {
        self.pressed
    }
}
