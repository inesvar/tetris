use super::assets::Assets;
use graphics::types::Matrix2d;
use graphics::{Context, DrawState};
use include_assets::NamedArchive;
use opengl_graphics::{GlGraphics, OpenGL};

pub struct Piston2dOpenGlRenderer<'a> {
    pub gl: GlGraphics,
    pub assets: Assets<'a>,
    pub transform: Matrix2d,
    pub draw_state: DrawState,
    pub elapsed_secs: f64,
}

impl<'a> Piston2dOpenGlRenderer<'a> {
    pub fn new(gl_version: OpenGL, assets_archive: &'a NamedArchive) -> Self {
        let assets = Assets::new(assets_archive);

        Self {
            gl: GlGraphics::new(gl_version),
            assets,
            transform: Matrix2d::default(),
            draw_state: DrawState::default(),
            elapsed_secs: 0.0,
        }
    }

    pub fn update(&mut self, ctx: &Context, clock: f64) {
        self.transform = ctx.transform;
        self.draw_state = ctx.draw_state;
        self.elapsed_secs = clock;
    }
}
