use crate::assets::Assets;
use crate::settings::TEXT_COLOR;
use crate::ui::button::Button;
use crate::ui::interactive_widget_manager::InteractiveWidgetManager;
use crate::ui::key_input::KeyInput;
use crate::ui::text::Text;
use crate::ui::text_input::TextInput;
use graphics::types::Matrix2d;
use graphics::{color, rectangle, Context, Transformed};
use opengl_graphics::{GlGraphics, GlyphCache};

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

        let color = if self.focused { color::RED } else { TEXT_COLOR };

        let outline_rect = graphics::Rectangle::new_border(color, 1.0);
        outline_rect.draw(dims, &ctx.draw_state, button_transform, gl);

        if self.focused {
            if self.animation_counter % 60 == 0 {
                if self.cursor.len() == 0 {
                    self.cursor.push('|');
                } else {
                    self.cursor.pop();
                }
            }
        } else {
            self.cursor.clear();
        }

        self.text.content.push_str(&self.cursor);
        self.text.render(transform, ctx, gl, font);
        if self.cursor.len() > 0 {
            self.text.content.pop();
        }
        let info_transform = transform.trans(0.0, -50.0);
        self.info_text.render(info_transform, ctx, gl, font);
    }
}

impl KeyInput {
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

        let color = if self.focused { color::RED } else { TEXT_COLOR };

        let outline_rect = graphics::Rectangle::new_border(color, 1.0);
        outline_rect.draw(dims, &ctx.draw_state, button_transform, gl);

        if self.focused {
            // update the cursor so it appears to be blinking
            if self.animation_counter % 60 == 0 {
                if self.cursor.len() == 0 {
                    self.cursor.push('|');
                } else {
                    self.cursor.pop();
                }
            }
            // render the text temporarily with the cursor
            self.custom_text.content.push_str(&self.cursor);
            self.custom_text.render(transform, ctx, gl, font);
            if self.cursor.len() > 0 {
                self.custom_text.content.pop();
            }
        } else {
            self.cursor.clear();
            // if it's not focused, render the placeholder or custom text
            if self.custom {
                self.custom_text.render(transform, ctx, gl, font);
            } else {
                self.placeholder.render(transform, ctx, gl, font);
            }
        }

        // render the info_text
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
        for key_input in self.key_inputs.values_mut() {
            key_input.render(transform, ctx, gl, &mut assets.main_font);
        }
    }
}
