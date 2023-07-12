use crate::assets::Assets;
use crate::once;
use crate::settings::TEXT_COLOR;
use crate::ui::button::Button;
use crate::ui::interactive_widget_manager::ButtonType::NewSinglePlayerGame;
use crate::ui::interactive_widget_manager::InteractiveWidgetManager;
use crate::ui::text::Text;
use crate::ui::text_input::TextInput;
use graphics::types::Matrix2d;
use graphics::{color, rectangle, Context, Transformed};
use opengl_graphics::{GlGraphics, GlyphCache};
use serde::de::Unexpected::Str;

impl Text {
    pub fn render(
        &self,
        transform: Matrix2d,
        ctx: &Context,
        gl: &mut GlGraphics,
        font: &mut GlyphCache,
    ) {
        let text_transform = transform.trans(
            self.x - self.content.len() as f64 * self.font_size as f64 * 0.315,
            self.y + self.font_size as f64 * 0.41,
        );
        self.view
            .draw(
                self.content.as_str(),
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
        &mut self,
        transform: Matrix2d,
        ctx: &Context,
        gl: &mut GlGraphics,
        font: &mut GlyphCache,
    ) {
        self.animation_counter += 1;

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
            TEXT_COLOR
        };

        let outline_rect = graphics::Rectangle::new_border(color, 1.0);
        outline_rect.draw(dims, &ctx.draw_state, button_transform, gl);

        if self.get_focused() {
            if self.animation_counter % 60 == 0 {
                if self.text.content.contains("|") {
                    self.text.content = self.text.content.replace("|", "");
                } else {
                    self.text.content.push('|');
                }
            } else {
                if self.text.content.contains("|") {
                    self.text.content = self.text.content.replace("|", "");
                    self.text.content.push('|');
                }
            }
        }

        self.text.render(transform, ctx, gl, font);
        let info_transform = transform.trans(0.0, -50.0);
        self.info_text.render(info_transform, ctx, gl, font);
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
