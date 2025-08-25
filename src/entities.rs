use coffee::graphics::{Frame, Rectangle, Sprite, Point};
use crate::assets::*;
use crate::drawing_utils::*;

use crate::WINDOW_HEIGHT;

pub enum Action {
    Idle,
    Left,
    Right,
    Up,
    Down,
    Shoot,
}
pub struct Player {
    position: Rectangle<f32>,
    velocity: f32,
    on_ground: bool,
}

impl Player {
    pub const SPEED: f32 = 500.0;
    pub const JUMP: f32 = 10.0;
    pub const GRAVITY: f32 = 3.0;

    pub fn new(position: Rectangle<f32>) -> Self {
        Player {
            position,
            velocity: 0.0,
            on_ground: true,
        }
    }

    pub fn update(&mut self, action: Action, seconds: f32) {
        match action {
            Action::Left => {
                self.position.x = self.position.x - seconds * Self::SPEED;
                self.position.x = f32::clamp(self.position.x, 0.0, crate::WINDOW_WIDTH);
            }
            Action::Right => {
                self.position.x = self.position.x + seconds * Self::SPEED;
                self.position.x = f32::clamp(self.position.x, 0.0, crate::WINDOW_WIDTH);
            }
            Action::Up => {
                if self.on_ground {
                    self.velocity = Self::JUMP;
                    self.on_ground = false;
                }
            }
            _ => (),
        }

        self.velocity += Self::GRAVITY * seconds;

        self.position.y += self.velocity * seconds;
        self.position.y = f32::clamp(self.position.y, 0.0, WINDOW_HEIGHT)
    }

    pub fn draw(&mut self, frame: &mut Frame, assets: &Assets, player_drawing_utils: &PlayerSpriteSlices) {
        let player_sprite: Sprite = Sprite {
            source: player_drawing_utils.Idle,
            position: Point::new(self.position.x,self.position.y),
            scale: player_drawing_utils.scale
        };

        assets.player_sprite_sheet.draw(player_sprite, &mut frame.as_target());
    }
}
