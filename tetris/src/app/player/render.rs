//! Define `trait` [Render] for [LocalPlayer], [TetrisPlayer], [Tetromino], [TetrisGrid].
use crate::app::player::LocalPlayer;
use crate::app::remote::RemotePlayer;
use crate::app::render_app::Piston2dGraphicsArguments;
use crate::app::ui::render_text;
use crate::app::ui::text::Text;
use crate::app::TetrisPlayer;
use crate::once;
use crate::settings::{
    BLOCK_SIZE, DEFAULT_FONT_SIZE, DEFAULT_GRID_X, DEFAULT_GRID_Y, DEFAULT_SCORE_TEXT_Y,
    GRID_BG_COLOR, GRID_COLOR, GRID_THICKNESS, NB_NEXT_TETROMINO, TETROMINO_MAX_HEIGHT,
    TETROMINO_MAX_WIDTH, TEXT_COLOR,
};
use graphics::types::{Rectangle, Scalar};
use graphics::{rectangle, Image, Transformed};
use tetris_core::{Position, TetrisColor, TetrisGrid, Tetromino, NB_VISIBLE_BUFFER_ROWS};

pub fn render_remote_player(remote_player: &RemotePlayer, gl_ctx: &mut Piston2dGraphicsArguments) {
    if !remote_player.received_first_screen() {
        return;
    }
    {
        let screen = remote_player.get_player();
        render_player(&screen, gl_ctx);
    }
    once!("render was done");
}

pub fn render_local_player(local_player: &LocalPlayer, gl_ctx: &mut Piston2dGraphicsArguments) {
    render_player(&local_player.player_screen, gl_ctx);
}

fn render_player(player: &TetrisPlayer, gl_ctx: &mut Piston2dGraphicsArguments) {
    let score_text = Text::new(
        format!("Score: {}", player.score).as_str(),
        DEFAULT_FONT_SIZE,
        // the score is centered under the hold piece rectangle
        DEFAULT_GRID_X - 4.0 * BLOCK_SIZE, // 4.0 = 1.0 (margin between borders) + 1.0 (margin inside) + 2.0 (half TETROMINO_MAX_WIDTH)
        DEFAULT_SCORE_TEXT_Y,
        TEXT_COLOR,
    );
    render_text(&score_text, gl_ctx);

    let old_transform = gl_ctx.transform;
    let grid_transform = gl_ctx.transform.trans(DEFAULT_GRID_X, DEFAULT_GRID_Y);
    gl_ctx.transform = grid_transform;

    if !player.is_in_play() {
        render_tetromino(&player.tetromino_in_play, gl_ctx);
    }

    render_tetris_grid(&player.matrix, gl_ctx);

    if player.is_in_play() {
        render_tetromino(&player.tetromino_in_play, gl_ctx);

        let old_draw_state = gl_ctx.draw_state;
        gl_ctx.draw_state = gl_ctx
            .draw_state
            .blend(graphics::draw_state::Blend::Multiply);
        render_tetromino(&player.get_ghost_tetromino(), gl_ctx);
        gl_ctx.draw_state = old_draw_state;
    }

    // drawing a border for the hold piece
    gl_ctx.transform = grid_transform.trans(
        -(BLOCK_SIZE + TETROMINO_MAX_WIDTH + BLOCK_SIZE + BLOCK_SIZE),
        hidden_height(&player.matrix),
    );
    let rectangle_width = BLOCK_SIZE + TETROMINO_MAX_WIDTH + BLOCK_SIZE;
    let rectangle_height = BLOCK_SIZE + TETROMINO_MAX_HEIGHT + BLOCK_SIZE;
    let dims: Rectangle = [0.0, 0.0, rectangle_width, rectangle_height];
    rectangle(GRID_BG_COLOR, dims, gl_ctx.transform, gl_ctx.gl);
    let outline_rect = graphics::Rectangle::new_border(GRID_COLOR, GRID_THICKNESS);
    outline_rect.draw(dims, &gl_ctx.draw_state, gl_ctx.transform, gl_ctx.gl);

    // drawing the hold piece
    if let Some(saved) = &player.hold_queue {
        gl_ctx.transform = grid_transform.trans(
            -TETROMINO_MAX_WIDTH - 2.0 * BLOCK_SIZE,
            TETROMINO_MAX_HEIGHT + BLOCK_SIZE,
        );
        render_tetromino(saved, gl_ctx);
    }

    // drawing a border for the fifo of next pieces
    gl_ctx.transform = grid_transform.trans(
        total_width(&player.matrix) + BLOCK_SIZE,
        hidden_height(&player.matrix),
    );
    let width = BLOCK_SIZE + TETROMINO_MAX_WIDTH + BLOCK_SIZE;
    let height = BLOCK_SIZE + (BLOCK_SIZE + TETROMINO_MAX_HEIGHT) * NB_NEXT_TETROMINO as f64;
    let dims: Rectangle = [0.0, 0.0, width, height];
    rectangle(GRID_BG_COLOR, dims, gl_ctx.transform, gl_ctx.gl);
    let outline_rect = graphics::Rectangle::new_border(GRID_COLOR, GRID_THICKNESS);
    outline_rect.draw(dims, &gl_ctx.draw_state, gl_ctx.transform, gl_ctx.gl);

    // drawing the next pieces
    for i in 0..NB_NEXT_TETROMINO {
        gl_ctx.transform = grid_transform.trans(
            total_width(&player.matrix) + 2.0 * BLOCK_SIZE,
            (BLOCK_SIZE + TETROMINO_MAX_HEIGHT) * (i as f64 + 1.0),
        );
        if let Some(tetromino) = player.next_queue.get(i) {
            render_tetromino(tetromino, gl_ctx);
        }
    }

    gl_ctx.transform = old_transform;
}

fn render_tetris_grid(grid: &TetrisGrid, gl_ctx: &mut Piston2dGraphicsArguments<'_, '_>) {
    let empty_dims: Rectangle = [
        0.0,
        hidden_height(grid),
        total_width(grid),
        visible_height(grid),
    ];
    rectangle(GRID_BG_COLOR, empty_dims, gl_ctx.transform, gl_ctx.gl);
    let outline_rect = graphics::Rectangle::new_border(GRID_COLOR, GRID_THICKNESS * 2.0);
    outline_rect.draw(empty_dims, &gl_ctx.draw_state, gl_ctx.transform, gl_ctx.gl);

    for position in grid.positions() {
        if let Some(tetris_color) = grid[&position] {
            render_tetris_block(&position, &tetris_color, gl_ctx);
        }
    }
}

fn render_tetromino(tetromino: &Tetromino, gl_ctx: &mut Piston2dGraphicsArguments<'_, '_>) {
    for block in tetromino.blocks() {
        render_tetris_block(block, &tetromino.color(), gl_ctx);
    }
}

/// In [player::render](super::render), helper to render a tetris block at `position` with `tetris_color`.
fn render_tetris_block(
    position: &Position,
    tetris_color: &TetrisColor,
    gl_ctx: &mut Piston2dGraphicsArguments<'_, '_>,
) {
    let dims = rectangle::square(
        position.x() as Scalar * BLOCK_SIZE,
        position.y() as Scalar * BLOCK_SIZE,
        BLOCK_SIZE,
    );

    Image::new().rect(dims).draw(
        gl_ctx.assets.texture_for(tetris_color),
        &gl_ctx.draw_state,
        gl_ctx.transform,
        gl_ctx.gl,
    );
}

fn total_width(grid: &TetrisGrid) -> f64 {
    grid.nb_columns_i32() as f64 * BLOCK_SIZE
}

fn visible_height(grid: &TetrisGrid) -> f64 {
    grid.nb_matrix_rows_i32() as f64 * BLOCK_SIZE
}

fn hidden_height(_grid: &TetrisGrid) -> f64 {
    NB_VISIBLE_BUFFER_ROWS as f64 * BLOCK_SIZE
}
