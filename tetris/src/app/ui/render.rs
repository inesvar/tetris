//! Implement [Render] for [Text] and [InteractiveWidgetManager] (composed of [TextInput], [KeyInput] and [Button]).
use super::super::Piston2dOpenGlRenderer;
use super::button::Button;
use super::interactive_widget_manager::InteractiveWidgetManager;
use super::key_input::KeyInput;
use super::text::Text;
use super::text_input::TextInput;
use crate::settings::{CURSOR_BLINK_PERIOD, DEFAULT_BUTTON_Y_SPACING, TEXT_COLOR};
use graphics::{color, rectangle, Transformed};

pub fn render_text(text: &Text, gl_ctx: &mut Piston2dOpenGlRenderer) {
    let old_transform = gl_ctx.transform;
    gl_ctx.transform = gl_ctx.transform.trans(
        text.x - text.content.len() as f64 * text.font_size as f64 * 0.315,
        text.y + text.font_size as f64 * 0.41,
    );

    let font = if text.use_tetris_font {
        &mut gl_ctx.assets.tetris_font
    } else {
        &mut gl_ctx.assets.main_font
    };
    text.view
        .draw(
            text.content.as_str(),
            font,
            &gl_ctx.draw_state,
            gl_ctx.transform,
            &mut gl_ctx.gl,
        )
        .unwrap();

    gl_ctx.transform = old_transform;
}

fn render_text_input(input: &TextInput, gl_ctx: &mut Piston2dOpenGlRenderer) {
    let dims = rectangle::rectangle_by_corners(
        -input.width / 2.0,
        -input.height / 2.0,
        input.width / 2.0,
        input.height / 2.0,
    );
    let button_transform = gl_ctx.transform.trans(input.x, input.y);

    let color = if input.focused {
        color::RED
    } else {
        TEXT_COLOR
    };

    let outline_rect = graphics::Rectangle::new_border(color, 1.0);
    outline_rect.draw(dims, &gl_ctx.draw_state, button_transform, &mut gl_ctx.gl);

    if input.focused && gl_ctx.elapsed_secs % CURSOR_BLINK_PERIOD < CURSOR_BLINK_PERIOD / 2.0 {
        let mut text_with_cursor = input.text.clone();
        text_with_cursor.content.push('|');
        render_text(&text_with_cursor, gl_ctx);
    } else {
        render_text(&input.text, gl_ctx);
    }

    let old_transform = gl_ctx.transform;
    gl_ctx.transform = gl_ctx.transform.trans(0.0, -DEFAULT_BUTTON_Y_SPACING / 2.0);
    render_text(&input.info_text, gl_ctx);

    gl_ctx.transform = old_transform;
}

fn render_key_input(key_input: &KeyInput, gl_ctx: &mut Piston2dOpenGlRenderer) {
    let dims = rectangle::rectangle_by_corners(
        -key_input.width / 2.0,
        -key_input.height / 2.0,
        key_input.width / 2.0,
        key_input.height / 2.0,
    );
    let button_transform = gl_ctx.transform.trans(key_input.x, key_input.y);

    let color = if key_input.focused {
        color::RED
    } else {
        TEXT_COLOR
    };

    let outline_rect = graphics::Rectangle::new_border(color, 1.0);
    outline_rect.draw(dims, &gl_ctx.draw_state, button_transform, &mut gl_ctx.gl);

    if key_input.focused && gl_ctx.elapsed_secs % CURSOR_BLINK_PERIOD < CURSOR_BLINK_PERIOD / 2.0 {
        let mut text_with_cursor = key_input.custom_text.clone();
        text_with_cursor.content.push('|');
        render_text(&text_with_cursor, gl_ctx);
    } else if key_input.focused || key_input.custom {
        render_text(&key_input.custom_text, gl_ctx);
    } else {
        render_text(&key_input.placeholder, gl_ctx);
    }

    // render the info_text
    let old_transform = gl_ctx.transform;
    gl_ctx.transform = gl_ctx.transform.trans(0.0, -DEFAULT_BUTTON_Y_SPACING / 2.0);
    render_text(&key_input.info_text, gl_ctx);

    gl_ctx.transform = old_transform;
}

fn render_button(button: &Button, gl_ctx: &mut Piston2dOpenGlRenderer) {
    let dims = rectangle::rectangle_by_corners(
        -button.width / 2.0,
        -button.height / 2.0,
        button.width / 2.0,
        button.height / 2.0,
    );
    let rectangle = graphics::Rectangle::new(button.background_color);

    let old_transform = gl_ctx.transform;
    gl_ctx.transform = gl_ctx.transform.trans(button.x, button.y);

    rectangle.draw(dims, &gl_ctx.draw_state, gl_ctx.transform, &mut gl_ctx.gl);

    render_text(&button.text, gl_ctx);

    gl_ctx.transform = old_transform;
}

pub fn render_widget_manager(
    manager: &InteractiveWidgetManager,
    gl_ctx: &mut Piston2dOpenGlRenderer,
) {
    for button in manager.buttons.values() {
        render_button(button, gl_ctx);
    }

    for text_input in manager.text_inputs.values() {
        render_text_input(text_input, gl_ctx);
    }
    for key_input in manager.key_inputs.values() {
        render_key_input(key_input, gl_ctx);
    }
}
