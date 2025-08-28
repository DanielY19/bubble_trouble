//use std::ops::Range;

use coffee::graphics::{Frame, Mesh, Point, Rectangle, Sprite, Shape, Color};
use crate::{assets::*, parameter_generator::ParameterGenerator, WINDOW_HEIGHT, WINDOW_WIDTH};
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
    pub velocity: (f32, f32),
    pub on_ground: bool,
}

impl Player {
    const PLAYER_GRAVITY: f32 = 300.0;
    const SPEED: f32 = 200.0;
    const JUMP: f32 = 350.0;

    pub fn new(position: Rectangle<f32>) -> Player {
        Player {
            position,
            velocity: (Player::SPEED,Player::JUMP),
            on_ground: true,
        }
    }

    pub fn update(&mut self, action: Action, seconds: f32) {
        match action {
            Action::Left | Action::Right => {
                let mut dx = 1.0;

                if let Action::Left = action {
                    dx = -1.0;
                }

                self.position.x += seconds * self.velocity.0 * dx;
                self.position.x = f32::clamp(self.position.x, 0.0, WINDOW_WIDTH - self.position.width);
            }
            Action::Up => {
                if self.on_ground {
                    self.velocity.1 = Player::JUMP;
                    self.on_ground = false;
                }
            }
            _ => if !self.on_ground {
                self.velocity.1 -= Player::PLAYER_GRAVITY * seconds;
                self.position.y -= self.velocity.1 * seconds;
                self.position.y = f32::clamp(self.position.y, 0.0, WINDOW_HEIGHT);
            },
        }
    }

    pub fn collide_with_platform_horizontal(&mut self) {
        self.velocity.0 = 0.0; 
        self.on_ground = false;
    }

    pub fn collide_with_platform_top(&mut self) {
        self.on_ground = true;
    }

    pub fn collide_with_platform_bottom(&mut self) {
        self.velocity.1 = 0.0;
        self.on_ground = false;
    }

    pub fn no_collision(&mut self) {
        self.velocity.0 = Player::SPEED;
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

pub struct Platform {
    pub position: Rectangle<f32>,
}

impl Platform {
    pub const GROUND_HEIGHT: f32 = 30.0;
    const DIFF_ELEVATION: f32 = 75.0;
    //const TOP_ELEVATION: f32 = 100.0;

    pub fn new(position: Rectangle<f32>) -> Platform {
        Platform{ position }
    }

    pub fn ground() -> Platform {
        let ground_position = Point::new(0.0,WINDOW_HEIGHT - Platform::GROUND_HEIGHT);

        let ground_params = Rectangle {
                x:ground_position.x,
                y:ground_position.y,
                width:WINDOW_WIDTH,
                height:Platform::GROUND_HEIGHT
         };

        Platform::new(ground_params)
    }

    pub fn elevations() -> Vec<f32> {
        let relative_window_height = WINDOW_HEIGHT - Platform::GROUND_HEIGHT;

        vec![
            relative_window_height / 1.5 + Platform::DIFF_ELEVATION,
            relative_window_height / 3.0 + Platform::DIFF_ELEVATION,
            //Platform::TOP_ELEVATION .. relative_window_height / 3.0 - Platform::DIFF_ELEVATION
            ]
    }

    pub fn generate_platforms(parameter_generator: &mut ParameterGenerator) -> Vec<Platform> {
        let ground = Platform::ground();
        let mut platforms = vec![ground];

        Platform::elevations().into_iter().for_each(|elevation_range| {
            platforms.push(Platform::new(parameter_generator.generate_platform(elevation_range)));
        });

        platforms
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
    const BUBBLE_STROKE_WIDTH: f32 = 2.0;
    const ENERGY_LOSS:f32 = 0.99;
    const BUBBLE_GRAVITY:f32 = 100.0;
    const BUBBLE_RADIUS_MINIMUM:f32 = 25.0;

    pub fn new(position: Rectangle<f32>, velocity: (f32,f32)) -> Bubble {
        let radius = position.width / 2.0;
        Bubble{ position, radius, velocity }
    }

    pub fn update(&mut self, seconds: f32) {
        self.velocity.1 += Bubble::BUBBLE_GRAVITY * seconds;

        self.position.x += self.velocity.0 * seconds;
        self.position.y += self.velocity.1 * seconds;
    }

    pub fn pop(&self) -> Option<Vec<Bubble>> {
        let mut position = self.position;
        position.width /= 2.0;
        position.height /= 2.0;

        (position.width >= Bubble::BUBBLE_RADIUS_MINIMUM).then(|| {
            vec![
            Bubble::new(position,(self.velocity.0,self.velocity.1)),
            Bubble::new(position,(-self.velocity.0,self.velocity.1)),
            ]
        })
    }

    pub fn bounce_off_wall(&mut self) {
        self.velocity.0 *= -1.0;
    }

    pub fn bounce_off_ceiling(&mut self) {
        self.velocity.1 *= -1.0;
    }

    pub fn bounce_off_ground(&mut self) {
        self.velocity.1 *= -Bubble::ENERGY_LOSS;
    }

    pub fn draw(&self, frame: &mut Frame, color: Color) {
        let mut shape = Mesh::new();
        let center = Point::new(self.position.center().x,self.position.center().y);
        shape.stroke(Shape::Circle {center,radius:self.radius}, color,Bubble::BUBBLE_STROKE_WIDTH);
        shape.draw(&mut frame.as_target());
    }
}

pub enum HarpoonState {
    Active,
    Stationary,
    Inactive
}

pub struct Harpoon {
    pub position: Rectangle<f32>,
    pub state: HarpoonState,
}

impl Harpoon {
    pub const HARPOON_VELOCITY: f32 = 200.0;

    pub fn new(position: Rectangle<f32>, state: HarpoonState) -> Harpoon {
        Harpoon { position ,state }
    }

    pub fn fire(&mut self, position: &Rectangle<f32>) {
        if let HarpoonState::Inactive = self.state {
            self.position.x = position.center().x - self.position.width;
            self.position.y = position.y + position.height;
            self.state = HarpoonState::Active;
        }
    }

    pub fn hit_bubble(&mut self) {
        self.state = HarpoonState::Inactive;
    }

    pub fn hit_platform(&mut self) {
        if let HarpoonState::Active = self.state {
            self.state = HarpoonState::Stationary;
        }
    }

    pub fn update(&mut self, seconds: f32) {
        if let HarpoonState::Active = self.state {
            self.position.y -= Harpoon::HARPOON_VELOCITY * seconds;
            self.position.height += Harpoon::HARPOON_VELOCITY * seconds;
        }
    }

    pub fn draw(&self, frame: &mut Frame, assets: &Assets,) {
        match self.state {
            HarpoonState::Active | HarpoonState::Stationary => {
                let harpoon_sprite: Sprite = Sprite {
                    source: Rectangle { x:0,y:0,width:assets.harpoon_sprite_sheet.width(),height: self.position.height as u16 },
                    position: Point::new(self.position.x,self.position.y),
                    scale: (1.0,1.0)
                };

                assets.harpoon_sprite_sheet.draw(harpoon_sprite, &mut frame.as_target());
            }
            _ => ()
        }
    }
}