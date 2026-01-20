//! Implement [Render] for [Text] and [InteractiveWidgetManager] (composed of [TextInput], [KeyInput] and [Button]).
use super::super::Piston2dGraphicsArguments;
use super::button::Button;
use super::interactive_widget_manager::InteractiveWidgetManager;
use super::key_input::KeyInput;
use super::text::Text;
use super::text_input::TextInput;
use crate::{
    app::render_app::Render,
    settings::{CURSOR_BLINK_PERIOD, DEFAULT_BUTTON_Y_SPACING, TEXT_COLOR},
};
use graphics::{color, rectangle, Transformed};

impl Render<Piston2dGraphicsArguments<'_>> for TextInput {
    fn render(&self, gl_ctx: &mut Piston2dGraphicsArguments) {
        let dims = rectangle::rectangle_by_corners(
            -self.width / 2.0,
            -self.height / 2.0,
            self.width / 2.0,
            self.height / 2.0,
        );
        let button_transform = gl_ctx.transform.trans(self.x, self.y);

        let color = if self.focused { color::RED } else { TEXT_COLOR };

        let outline_rect = graphics::Rectangle::new_border(color, 1.0);
        outline_rect.draw(dims, &gl_ctx.draw_state, button_transform, gl_ctx.gl);

        let old_transform = gl_ctx.transform;

        gl_ctx.transform = old_transform;
    }
}

impl Render<Piston2dGraphicsArguments<'_>> for KeyInput {
    fn render(&self, gl_ctx: &mut Piston2dGraphicsArguments) {
        let dims = rectangle::rectangle_by_corners(
            -self.width / 2.0,
            -self.height / 2.0,
            self.width / 2.0,
            self.height / 2.0,
        );
        let button_transform = gl_ctx.transform.trans(self.x, self.y);

        let color = if self.focused { color::RED } else { TEXT_COLOR };

        let outline_rect = graphics::Rectangle::new_border(color, 1.0);
        outline_rect.draw(dims, &gl_ctx.draw_state, button_transform, gl_ctx.gl);

        // render the info_text
        let old_transform = gl_ctx.transform;

        gl_ctx.transform = old_transform;
    }
}

impl Render<Piston2dGraphicsArguments<'_>> for Button {
    fn render(&self, gl_ctx: &mut Piston2dGraphicsArguments) {
        let dims = rectangle::rectangle_by_corners(
            -self.width / 2.0,
            -self.height / 2.0,
            self.width / 2.0,
            self.height / 2.0,
        );
        let button = graphics::Rectangle::new(self.background_color);

        let old_transform = gl_ctx.transform;
        gl_ctx.transform = gl_ctx.transform.trans(self.x, self.y);

        button.draw(dims, &gl_ctx.draw_state, gl_ctx.transform, gl_ctx.gl);

        gl_ctx.transform = old_transform;
    }
}

impl Render<Piston2dGraphicsArguments<'_>> for InteractiveWidgetManager {
    fn render(&self, gl_ctx: &mut Piston2dGraphicsArguments) {
        for button in self.buttons.values() {
            button.render(gl_ctx);
        }

        for text_input in self.text_inputs.values() {
            text_input.render(gl_ctx);
        }
        for key_input in self.key_inputs.values() {
            key_input.render(gl_ctx);
        }
    }
}
