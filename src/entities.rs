use coffee::graphics::{Frame, Mesh, Point, Rectangle, Sprite, Shape, Color};
use crate::{assets::*,WINDOW_WIDTH,WINDOW_HEIGHT};

pub const GRAVITY: f32 = 3.0;
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
                self.position.x -= seconds * Self::SPEED;
                self.position.x = f32::clamp(self.position.x, 0.0, crate::WINDOW_WIDTH);
            }
            Action::Right => {
                self.position.x += seconds * Self::SPEED;
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

        self.velocity += GRAVITY * seconds;

        self.position.y += self.velocity * seconds;
        self.position.y = f32::clamp(self.position.y, 0.0, WINDOW_HEIGHT)
    }

    pub fn draw(&self, frame: &mut Frame, assets: &Assets, motion: Rectangle<u16>) {
        let player_sprite: Sprite = Sprite {
            source: motion,
            position: Point::new(self.position.x,self.position.y),
            scale: assets.player_sprite_slices.scale
        };

        assets.player_sprite_sheet.draw(player_sprite, &mut frame.as_target());
    }
}

pub struct Platform{
    pub position: Rectangle<f32>,
}

impl Platform{
    pub const GROUND_HEIGHT: f32 = 30.0;

    pub fn new(position: Rectangle<f32>) -> Platform {
        Platform{ position }
    }

    pub fn ground() -> Platform {
        let ground_position = Point::new(0.0,WINDOW_HEIGHT - Self::GROUND_HEIGHT);

        let ground_params = Rectangle {
                x:ground_position.x,
                y:ground_position.y,
                width:WINDOW_WIDTH,
                height:Self::GROUND_HEIGHT
         };

        Platform::new(ground_params)
    }

    pub fn draw(&self, frame: &mut Frame) {
        let mut mesh_fill = Mesh::new();
        mesh_fill.fill(Shape::Rectangle(self.position), Assets::WHITE);

        let mut mesh_stroke = Mesh::new();
        mesh_stroke.stroke(Shape::Rectangle(self.position), Assets::BLACK, Assets::MESH_STROKE_WIDTH);

        mesh_fill.draw(&mut frame.as_target());
        mesh_stroke.draw(&mut frame.as_target());
    }
}

pub struct Bubble {
    pub position: Rectangle<f32>,
    pub radius: f32,
    pub velocity: (f32,f32)
}

impl Bubble{
    pub const BUBBLE_STROKE_WIDTH: f32 = 2.0;

    pub fn new(position: Rectangle<f32>, velocity: (f32,f32)) -> Bubble {
        let radius = position.width / 2.0;
        Bubble{ position, radius, velocity }
    }

    pub fn update(&mut self, seconds: f32) {
        self.velocity.1 += GRAVITY * seconds;

        self.position.x += self.velocity.0 * seconds;
        self.position.y += self.velocity.1 * seconds;

        if self.position.x + self.radius <= self.radius || self.position.x + self.radius >= WINDOW_WIDTH - self.radius {
             self.velocity.0 = -self.velocity.0;
        }
    }

    pub fn pop(self) -> Vec<Bubble> {
        let mut position = self.position;
        position.width /= 2.0;
        position.height /= 2.0;

        vec![
            Bubble::new(position,(self.velocity.0,self.velocity.1)),
            Bubble::new(position,(-self.velocity.0,self.velocity.1)),
            ]
    }

    pub fn bounce_off_platform(&mut self, platform: &Platform) {

    }

    pub fn bounce_off_ground(&mut self, ground: &Platform) {

    }

    pub fn draw(&self, frame: &mut Frame, color: Color) {
        let mut shape = Mesh::new();
        let center = Point::new(self.position.x + self.position.width / 2.0,self.position.y + self.position.height / 2.0);
        shape.stroke(Shape::Circle {center,radius:self.radius}, color,Self::BUBBLE_STROKE_WIDTH);
        shape.draw(&mut frame.as_target());
    }
}