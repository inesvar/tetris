//! Assets include Tetromino block textures and fonts.
use include_assets::NamedArchive;
use opengl_graphics::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub enum TetrisColor {
    Cyan,
    Yellow,
    Purple,
    Blue,
    Orange,
    Green,
    Red,
    Grey,
}

pub struct Assets<'a> {
    pub cyan_texture: Texture,
    pub yellow_texture: Texture,
    pub purple_texture: Texture,
    pub blue_texture: Texture,
    pub orange_texture: Texture,
    pub green_texture: Texture,
    pub red_texture: Texture,
    pub grey_texture: Texture,
    pub sprite_sheet_texture: Texture,
    pub tetris_font: GlyphCache<'a>,
    pub main_font: GlyphCache<'a>,
}

impl<'a> Assets<'a> {
    pub fn new(assets: &'a NamedArchive) -> Self {
        let cyan_bytes = assets.get("textures/cyan.bmp").unwrap();
        let yellow_bytes = assets.get("textures/yellow.bmp").unwrap();
        let purple_bytes = assets.get("textures/purple.bmp").unwrap();
        let blue_bytes = assets.get("textures/blue.bmp").unwrap();
        let orange_bytes = assets.get("textures/orange.bmp").unwrap();
        let green_bytes = assets.get("textures/green.bmp").unwrap();
        let red_bytes = assets.get("textures/red.bmp").unwrap();
        let grey_bytes = assets.get("textures/grey.bmp").unwrap();
        let sprite_sheet_bytes = assets.get("textures/sprite_sheet.bmp").unwrap();

        let cyan_texture = Texture::from_bytes(cyan_bytes, &TextureSettings::new()).unwrap();
        let yellow_texture = Texture::from_bytes(yellow_bytes, &TextureSettings::new()).unwrap();
        let purple_texture = Texture::from_bytes(purple_bytes, &TextureSettings::new()).unwrap();
        let blue_texture = Texture::from_bytes(blue_bytes, &TextureSettings::new()).unwrap();
        let orange_texture = Texture::from_bytes(orange_bytes, &TextureSettings::new()).unwrap();
        let green_texture = Texture::from_bytes(green_bytes, &TextureSettings::new()).unwrap();
        let red_texture = Texture::from_bytes(red_bytes, &TextureSettings::new()).unwrap();
        let grey_texture = Texture::from_bytes(grey_bytes, &TextureSettings::new()).unwrap();
        let sprite_sheet_texture =
            Texture::from_bytes(sprite_sheet_bytes, &TextureSettings::new()).unwrap();

        let tetris_font_bytes = assets
            .get("fonts/tetris-blocks-font/TetrisBlocks-P99g.ttf")
            .unwrap();
        let main_font_bytes = assets.get("fonts/digitalt/Digitalt.otf").unwrap();

        let tetris_font =
            GlyphCache::from_bytes(tetris_font_bytes, (), TextureSettings::new()).unwrap();
        let main_font =
            GlyphCache::from_bytes(main_font_bytes, (), TextureSettings::new()).unwrap();

        Assets {
            cyan_texture,
            yellow_texture,
            purple_texture,
            blue_texture,
            orange_texture,
            green_texture,
            red_texture,
            grey_texture,
            sprite_sheet_texture,
            tetris_font,
            main_font,
        }
    }

    pub fn texture_from_tetris_color(&self, color: &TetrisColor) -> &Texture {
        match color {
            TetrisColor::Cyan => &self.cyan_texture,
            TetrisColor::Yellow => &self.yellow_texture,
            TetrisColor::Purple => &self.purple_texture,
            TetrisColor::Blue => &self.blue_texture,
            TetrisColor::Orange => &self.orange_texture,
            TetrisColor::Green => &self.green_texture,
            TetrisColor::Red => &self.red_texture,
            TetrisColor::Grey => &self.grey_texture,
        }
    }
}
