use crate::assets::Assets;
use crate::graphics::Transformed;
use graphics::{color, rectangle, Context};
use opengl_graphics::GlGraphics;
use piston::RenderArgs;

pub struct Button {
    pub x: i8,
    pub y: i8,
    pub width: i8,
    pub height: i8,
    pub text: String,
    pub pressed: bool,
}

impl Button {
    pub fn new(x: i8, y: i8, width: i8, height: i8, text: String) -> Self {
        Button {
            x,
            y,
            width,
            height,
            text,
            pressed: false,
        }
    }

    pub fn is_clicked(&self, x: i8, y: i8) -> bool {
        x >= self.x && x <= self.x + self.width && y >= self.y && y <= self.y + self.height
    }

    pub fn is_pressed(&self) -> bool {
        self.pressed
    }

    pub fn render(
        &mut self,
        _args: &RenderArgs,
        ctx: &Context,
        gl: &mut GlGraphics,
        assets: &mut Assets,
    ) {
        let dims = rectangle::rectangle_by_corners(
            self.x as f64,
            self.y as f64,
            (self.x + self.width) as f64,
            (self.y + self.height) as f64,
        );
        let button = graphics::Rectangle::new([0.0, 0.0, 0.0, 0.5]);

        button.draw(dims, &ctx.draw_state, ctx.transform, gl);

        let text_transform = ctx.transform.trans(
            self.x as f64 + self.width as f64 / 2.0 - 8.0,
            self.y as f64 + self.height as f64 / 2.0 + 8.0,
        );
        graphics::text::Text::new_color(color::WHITE, 16)
            .draw(
                "T",
                &mut assets.main_font,
                &ctx.draw_state,
                text_transform,
                gl,
            )
            .unwrap();
    }
}
