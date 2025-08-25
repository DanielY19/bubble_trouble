use coffee::graphics::{Color, Frame, Mesh, Point, Rectangle, Sprite, Shape};
use rand::rngs::ThreadRng;
use rand::Rng;
use crate::{assets::*, WINDOW_WIDTH};
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
    pub position: Rectangle<f32>,
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

    pub fn draw(&mut self, frame: &mut Frame, assets: &Assets, motion: Rectangle<u16>) {
        let player_sprite: Sprite = Sprite {
            source: motion,
            position: Point::new(self.position.x,self.position.y),
            scale: assets.player_sprite_slices.scale
        };

        assets.player_sprite_sheet.draw(player_sprite, &mut frame.as_target());
    }
}

pub struct Platform{
    pub position: Rectangle<f32>
}

impl Platform{
    pub fn new(position: Rectangle<f32>) -> Platform {
        Platform{ position }
    }

    pub fn draw(&mut self, frame: &mut Frame) {
        let shape = &mut Mesh::new();
        shape.fill(Shape::Rectangle(self.position), Color::new(1.0,1.0,1.0,1.0));
        shape.draw(&mut frame.as_target());
    }
}