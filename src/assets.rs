use coffee::load::{Task};
use coffee::graphics::{Image};
use std::path::{self, PathBuf};

pub struct Assets {
    pub player_sprite_sheet: Image,
    pub player_sprite_dimensions: (u16, u16)
}

impl Assets {
    pub fn load() -> Task<Assets> {
        let path = PathBuf::from("resources/hero.png");
        let player_spirte_sheet_task = Image::load(path);

        player_spirte_sheet_task.map(|image| Assets { 
            player_sprite_dimensions: (image.width(),image.height()),
            player_sprite_sheet: image
        })
    }
}
