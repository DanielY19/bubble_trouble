use coffee::load::{Join, Task};
use coffee::graphics::{Image, Color, Rectangle};
use std::path::{PathBuf};

pub struct Assets {
    pub player_sprite_sheet: Image,
    pub harpoon_sprite_sheet: Image,

    pub player_sprite_slices: PlayerSpriteSlices,
}

impl Assets {
    pub const WHITE:Color = Color{r:1.0,g:1.0,b:1.0,a:1.0};
    pub const GREY:Color = Color{r:0.5,g:0.5,b:0.5,a:1.0};
    pub const BLACK:Color = Color{r:0.0,g:0.0,b:0.0,a:1.0};
    pub const MESH_STROKE_WIDTH:f32 = 2.0;

    pub fn load() -> Task<Assets> {
        let path = PathBuf::from("resources/hero.png");
        let player_spirte_sheet_task = Image::load(path);
         
        let path = PathBuf::from("resources/harpoon.png");
        let harpoon_sprite_task = Image::load(path);

        let image_loading_task = (player_spirte_sheet_task,harpoon_sprite_task).join();

        image_loading_task.map(|sprites| Assets { 
            player_sprite_slices: PlayerSpriteSlices::new(sprites.0.width(),sprites.0.height()),
            player_sprite_sheet: sprites.0,
            harpoon_sprite_sheet: sprites.1
        })
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
    const PLAYER_SPRITE_SHEET_ROWS:u16 = 4;
    const PLAYER_SPRITE_SHEET_COLS:u16 = 3;

    pub fn new(width: u16, height: u16) -> Self {
        let frame_width = width / Self::PLAYER_SPRITE_SHEET_ROWS;
        let frame_height = height / Self::PLAYER_SPRITE_SHEET_COLS;

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