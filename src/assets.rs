use coffee::load::{Task};
use coffee::graphics::{Image, Mesh, Rectangle};
use rand::{rng, Rng};
use rand::rngs::ThreadRng;
use std::path::{PathBuf};
use crate::{WINDOW_HEIGHT,WINDOW_WIDTH};

pub struct Assets {
    pub player_sprite_sheet: Image,
    pub player_sprite_slices: PlayerSpriteSlices,
}

impl Assets {
    pub fn load() -> Task<Assets> {
        let path = PathBuf::from("resources/hero.png");
        let player_spirte_sheet_task = Image::load(path);

        player_spirte_sheet_task.map(|image| Assets { 
            player_sprite_slices: PlayerSpriteSlices::new(image.width(),image.height()),
            player_sprite_sheet: image,
        })
    }
}

pub struct PlatformGenerator{
    generator: ThreadRng
}

impl PlatformGenerator {
    pub fn new() -> PlatformGenerator {
        PlatformGenerator { generator: rng() }
    }

    pub fn generate_platform(&mut self) -> Rectangle<f32> {
        let x = self.generator.random_range(0.0 ..= WINDOW_WIDTH);
        let y = self.generator.random_range(0.0 ..= WINDOW_HEIGHT);
        let width:f32 = self.generator.random_range(100.0 .. 200.0);
        let height:f32 = 20.0;

        Rectangle { x,y,width,height }
    }
}

pub struct PlayerSpriteSlices {
    pub idle: Rectangle<u16>,
    pub left: Rectangle<u16>,
    pub right: Rectangle<u16>,
    pub left_step: Rectangle<u16>,
    pub right_step: Rectangle<u16>,
    pub scale: (f32, f32),
}

impl PlayerSpriteSlices {
    pub fn new(width: u16, height: u16) -> Self {
        let frame_width = width / 4;
        let frame_height = height / 3;

        Self {
            idle: Rectangle {
                x: 0 * frame_width,
                y: 2 * frame_height,
                width: frame_width,
                height: frame_height,
            },
            left: Rectangle {
                x: 0 * frame_width,
                y: 1 * frame_height,
                width: frame_width,
                height: frame_height,
            },
            left_step: Rectangle {
                x: 1 * frame_width,
                y: 1 * frame_height,
                width: frame_width,
                height: frame_height,
            },
            right: Rectangle {
                x: 1 * frame_width,
                y: 0 * frame_height,
                width: frame_width,
                height: frame_height,
            },
            right_step: Rectangle {
                x: 0 * frame_width,
                y: 0 * frame_height,
                width: frame_width,
                height: frame_height,
            },
            scale: (1.0, 1.0),
        }
    }
}
