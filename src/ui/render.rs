use crate::assets::Assets;
use crate::once;
use crate::ui::button::Button;
use crate::ui::interactive_widget_manager::InteractiveWidgetManager;
use crate::ui::text::Text;
use graphics::types::Matrix2d;
use graphics::{rectangle, Context, Transformed, color};
use opengl_graphics::{GlGraphics, GlyphCache};
use crate::ui::interactive_widget_manager::ButtonType::CreateSinglePlayerGameButton;
use crate::ui::text_input::TextInput;

impl Text {
    pub fn render(
        &self,
        transform: Matrix2d,
        ctx: &Context,
        gl: &mut GlGraphics,
        font: &mut GlyphCache,
    ) {
        let text_transform = transform.trans(
            self.x - self.text.len() as f64 * self.font_size as f64 * 0.315,
            self.y + self.font_size as f64 * 0.41,
        );
        self.view
            .draw(
                self.text.as_str(),
                font,
                &ctx.draw_state,
                text_transform,
                gl,
            )
            .unwrap();
    }
}

impl TextInput {
    pub fn render(
        &self,
        transform: Matrix2d,
        ctx: &Context,
        gl: &mut GlGraphics,
        font: &mut GlyphCache,
    ) {
        let dims = rectangle::rectangle_by_corners(
            -self.width / 2.0,
            -self.height / 2.0,
            self.width / 2.0,
            self.height / 2.0,
        );
        let button_transform = transform.trans(self.x, self.y);

        let color = if self.get_focused() {
            color::RED
        } else {
            color::WHITE
        };

        let outline_rect = graphics::Rectangle::new_border(color, 1.0);
        outline_rect.draw(dims, &ctx.draw_state, button_transform, gl);

        self.text
            .render(transform, ctx, gl, font);
    }
}

impl Button {
    pub fn render(
        &mut self,
        transform: Matrix2d,
        ctx: &Context,
        gl: &mut GlGraphics,
        assets: &mut Assets,
    ) {
        let dims = rectangle::rectangle_by_corners(
            -self.width / 2.0,
            -self.height / 2.0,
            self.width / 2.0,
            self.height / 2.0,
        );
        let button = graphics::Rectangle::new(self.background_color);

        let button_transform = transform.trans(self.x, self.y);

        button.draw(dims, &ctx.draw_state, button_transform, gl);

        self.text
            .render(button_transform, ctx, gl, &mut assets.main_font);
    }
}

impl InteractiveWidgetManager {
    pub fn render(
        &mut self,
        transform: Matrix2d,
        ctx: &Context,
        gl: &mut GlGraphics,
        assets: &mut Assets,
    ) {
        for button in self.buttons.values_mut() {
            button.render(transform, ctx, gl, assets);
        }

        for text_input in self.text_inputs.values_mut() {
            text_input.render(transform, ctx, gl, &mut assets.main_font);
        }
    }
}
