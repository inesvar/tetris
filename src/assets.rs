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

pub struct Assets {
    pub cyan: Texture,
    pub red: Texture,
    pub orange: Texture,
    pub purple: Texture,
    pub blue: Texture,
    pub green: Texture,
    pub yellow: Texture,
}

impl Assets {
    pub fn new(assets_folder: PathBuf) -> Assets {
        let cyan_file = assets_folder.join("cyan.jpg");
        let red_file = assets_folder.join("red.jpg");
        let blue_file = assets_folder.join("blue.jpg");
        let green_file = assets_folder.join("green.jpg");
        let yellow_file = assets_folder.join("yellow.jpg");
        let purple_file = assets_folder.join("purple.jpg");
        let orange_file = assets_folder.join("orange.jpg");

        let cyan_texture = Texture::from_path(
            &cyan_file, &TextureSettings::new()).unwrap();

        let red_texture = Texture::from_path(
            &red_file, &TextureSettings::new()).unwrap();

        let blue_texture = Texture::from_path(
            &blue_file, &TextureSettings::new()).unwrap();

        let green_texture = Texture::from_path(
            &green_file, &TextureSettings::new()).unwrap();

        let yellow_texture = Texture::from_path(
            &yellow_file, &TextureSettings::new()).unwrap();

        let purple_texture = Texture::from_path(
            &purple_file, &TextureSettings::new()).unwrap();

        let orange_texture = Texture::from_path(
            &orange_file, &TextureSettings::new()).unwrap();

        Assets {
            cyan: cyan_texture,
            red: red_texture,
            blue: blue_texture,
            green: green_texture,
            yellow: yellow_texture,
            purple: purple_texture,
            orange: orange_texture,
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