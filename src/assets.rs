use coffee::load::{Task};
use coffee::graphics::{Image, Rectangle};
use std::path::{self, PathBuf};

pub struct Assets {
    pub player_sprite_sheet: Image,
    pub player_sprite_slices: PlayerSpriteSlices
}

impl Assets {
    pub fn load() -> Task<Assets> {
        let path = PathBuf::from("resources/hero.png");
        let player_spirte_sheet_task = Image::load(path);

        player_spirte_sheet_task.map(|image| Assets { 
            player_sprite_slices: PlayerSpriteSlices::new(image.width(),image.height()),
            player_sprite_sheet: image
        })
    }
}
pub struct PlayerSpriteSlices {
    pub Idle: Rectangle<u16>,
    pub Left: Rectangle<u16>,
    pub Right: Rectangle<u16>,
    pub LeftStep: Rectangle<u16>,
    pub RightStep: Rectangle<u16>,
    pub scale: (f32, f32),
}

impl PlayerSpriteSlices {
    pub fn new(width: u16, height: u16) -> Self {
        let frame_width = width / 4;
        let frame_height = height / 3;

        Self {
            Idle: Rectangle {
                x: 0 * frame_width,
                y: 2 * frame_height,
                width: frame_width,
                height: frame_height,
            },
            Left: Rectangle {
                x: 0 * frame_width,
                y: 1 * frame_height,
                width: frame_width,
                height: frame_height,
            },
            LeftStep: Rectangle {
                x: 1 * frame_width,
                y: 1 * frame_height,
                width: frame_width,
                height: frame_height,
            },
            Right: Rectangle {
                x: 1 * frame_width,
                y: 0 * frame_height,
                width: frame_width,
                height: frame_height,
            },
            RightStep: Rectangle {
                x: 0 * frame_width,
                y: 0 * frame_height,
                width: frame_width,
                height: frame_height,
            },
            scale: (1.0, 1.0),
        }
    }
}
