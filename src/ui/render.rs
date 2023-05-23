use graphics::{color, Context, Transformed};
use graphics::types::Matrix2d;
use opengl_graphics::{GlGraphics, GlyphCache};
use crate::assets::Assets;
use crate::ui::text::Text;

impl Text {
    pub fn render(&self, transform: Matrix2d, ctx: &Context, gl: &mut GlGraphics, font: &mut GlyphCache) {
        let title_transform = transform.trans(self.x, self.y);
        graphics::text::Text::new_color(color::WHITE, self.font_size)
            .draw(
                self.text.as_str(),
                font,
                &ctx.draw_state,
                title_transform,
                gl,
            )
            .unwrap();
    }
}