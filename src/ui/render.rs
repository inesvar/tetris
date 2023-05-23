use graphics::{Context, rectangle, Transformed};
use graphics::types::Matrix2d;
use opengl_graphics::{GlGraphics, GlyphCache};
use crate::assets::Assets;
use crate::ui::button::Button;
use crate::ui::text::Text;

impl Text {
    pub fn render(&self, transform: Matrix2d, ctx: &Context, gl: &mut GlGraphics, font: &mut GlyphCache) {
        let text_transform = transform.trans(self.x - self.text.len() as f64 * self.font_size as f64 / 4.0, self.y + self.font_size as f64 / 2.0);
        self.view.draw(
            self.text.as_str(),
            font,
            &ctx.draw_state,
            text_transform,
            gl,
        ).unwrap();
    }
}

impl Button {
    pub fn render(&mut self, transform: Matrix2d, ctx: &Context, gl: &mut GlGraphics, assets: &mut Assets) {
        let dims = rectangle::rectangle_by_corners(
            -self.width / 2.0,
            -self.height / 2.0,
            self.width / 2.0,
            self.height / 2.0,
        );
        let button = graphics::Rectangle::new([0.8, 0.8, 0.8, 1.0]);

        let button_transform = transform.trans(self.x, self.y);

        button.draw(dims, &ctx.draw_state, button_transform, gl);

        self.text.render(button_transform, ctx, gl, &mut assets.main_font);
    }
}