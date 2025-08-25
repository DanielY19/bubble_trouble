use coffee::graphics::{Image,Rectangle};
pub struct PlayerSpriteSlices {
    pub Idle: Rectangle<u16>,
    pub Left: Rectangle<u16>,
    pub Right: Rectangle<u16>,
    pub LeftStep: Rectangle<u16>,
    pub RightStep: Rectangle<u16>,
    pub scale: (f32, f32),
}

impl PlayerSpriteSlices {
    pub fn new(dimensions: (u16, u16)) -> Self {
        let frame_width = dimensions.0 / 4;
        let frame_height = dimensions.1 / 3;

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
