use std::path::PathBuf;
use opengl_graphics::*;

#[derive(Clone, Copy)]
pub enum TetrisColor {
    CYAN,
    YELLOW,
    RED,
    BLUE,
    ORANGE,
    PURPLE,
    GREEN,
}

impl TetrisColor {
    pub fn random() -> TetrisColor {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..7) {
            0 => TetrisColor::CYAN,
            1 => TetrisColor::YELLOW,
            2 => TetrisColor::RED,
            3 => TetrisColor::BLUE,
            4 => TetrisColor::ORANGE,
            5 => TetrisColor::PURPLE,
            6 => TetrisColor::GREEN,
            _ => panic!("random number out of range"),
        }
    }
}

pub struct Assets<'a> {
    pub cyan: Texture,
    pub red: Texture,
    pub orange: Texture,
    pub purple: Texture,
    pub blue: Texture,
    pub green: Texture,
    pub yellow: Texture,

    pub tetris_font: GlyphCache<'a>,
    pub main_font: GlyphCache<'a>
}

impl Assets<'_> {
    pub fn new(assets_folder: PathBuf) -> Assets<'static> {
        let texture_folder = assets_folder.join("textures");
        let cyan_file = texture_folder.join("cyan.jpg");
        let red_file = texture_folder.join("red.jpg");
        let blue_file = texture_folder.join("blue.jpg");
        let green_file = texture_folder.join("green.jpg");
        let yellow_file = texture_folder.join("yellow.jpg");
        let purple_file = texture_folder.join("purple.jpg");
        let orange_file = texture_folder.join("orange.jpg");

        let cyan_texture = Texture::from_path(
            cyan_file, &TextureSettings::new()).unwrap();

        let red_texture = Texture::from_path(
            red_file, &TextureSettings::new()).unwrap();

        let blue_texture = Texture::from_path(
            blue_file, &TextureSettings::new()).unwrap();

        let green_texture = Texture::from_path(
            green_file, &TextureSettings::new()).unwrap();

        let yellow_texture = Texture::from_path(
            yellow_file, &TextureSettings::new()).unwrap();

        let purple_texture = Texture::from_path(
            purple_file, &TextureSettings::new()).unwrap();

        let orange_texture = Texture::from_path(
            orange_file, &TextureSettings::new()).unwrap();

        let font_folder = assets_folder.join("fonts");

        let tetris_font_file = font_folder.join("tetris-blocks-font/TetrisBlocks-P99g.ttf");
        let main_font_file = font_folder.join("digitalt/Digitalt.otf");

        let tetris_font = GlyphCache::new(&tetris_font_file, (), TextureSettings::new()).unwrap();
        let main_font = GlyphCache::new(&main_font_file, (), TextureSettings::new()).unwrap();

        Assets {
            cyan: cyan_texture,
            red: red_texture,
            blue: blue_texture,
            green: green_texture,
            yellow: yellow_texture,
            purple: purple_texture,
            orange: orange_texture,

            tetris_font,
            main_font
        }
    }

    pub fn texture_from_tetris_color(&self, color: &TetrisColor) -> &Texture {
        match color {
            TetrisColor::BLUE => &self.blue,
            TetrisColor::CYAN => &self.cyan,
            TetrisColor::PURPLE => &self.purple,
            TetrisColor::GREEN => &self.green,
            TetrisColor::ORANGE => &self.orange,
            TetrisColor::RED => &self.red,
            TetrisColor::YELLOW => &self.yellow
        }
    }
}