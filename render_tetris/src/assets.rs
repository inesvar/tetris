//! Assets include tetromino block textures and fonts.
use core_tetris::TetrisColor;
use graphics::glyph_cache::rusttype::GlyphCache;
use include_assets::NamedArchive;
use opengl_graphics::*;
use std::path::{Path, PathBuf};

pub struct Assets<'a> {
    pub cyan_texture: Texture,
    pub yellow_texture: Texture,
    pub purple_texture: Texture,
    pub blue_texture: Texture,
    pub orange_texture: Texture,
    pub green_texture: Texture,
    pub red_texture: Texture,
    pub grey_texture: Texture,
    #[allow(unused)]
    pub sprite_sheet_texture: Texture,
    pub tetris_font: GlyphCache<'a, (), Texture>,
    pub main_font: GlyphCache<'a, (), Texture>,
}

const ASSETS_FOLDER: &str = "assets";
const TEXTURES_FOLDER: &str = "textures";
const TEXTURE_EXTENSION: &str = "bmp";

fn get_texture(filename: &str) -> Texture {
    let mut path: PathBuf = [ASSETS_FOLDER, TEXTURES_FOLDER].iter().collect();
    path.push(format!("{filename}.{TEXTURE_EXTENSION}"));

    Texture::from_path(&path, &TextureSettings::new())
        .expect("Provided path should contain texture")
}

fn get_font<'a>(archive: &'a NamedArchive, path: &Path) -> GlyphCache<'a, (), Texture> {
    let tetris_font_bytes = archive
        .get(&path.to_string_lossy())
        .expect("Path should contain archive");

    GlyphCache::from_bytes(tetris_font_bytes, (), TextureSettings::new())
        .expect("Bytes should contain valid font")
}

impl<'a> Assets<'a> {
    pub fn new(assets: &'a NamedArchive) -> Self {
        let cyan_texture = get_texture(TetrisColor::Cyan.get_texture_filename());
        let yellow_texture = get_texture(TetrisColor::Yellow.get_texture_filename());
        let purple_texture = get_texture(TetrisColor::Purple.get_texture_filename());
        let blue_texture = get_texture(TetrisColor::Blue.get_texture_filename());
        let orange_texture = get_texture(TetrisColor::Orange.get_texture_filename());
        let green_texture = get_texture(TetrisColor::Green.get_texture_filename());
        let red_texture = get_texture(TetrisColor::Red.get_texture_filename());
        let grey_texture = get_texture(TetrisColor::Grey.get_texture_filename());
        let sprite_sheet_texture = get_texture("sprite_sheet");

        let tetris_font_path: PathBuf = ["fonts", "tetris-blocks-font", "TetrisBlocks-P99g.ttf"]
            .iter()
            .collect();
        let tetris_font = get_font(assets, &tetris_font_path);

        let main_font_path: PathBuf = ["fonts", "digitalt", "Digitalt.otf"].iter().collect();
        let main_font = get_font(assets, &main_font_path);

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

    pub fn texture_for(&self, color: &TetrisColor) -> &Texture {
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
