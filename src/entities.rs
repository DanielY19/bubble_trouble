use coffee::graphics::{Frame, Mesh, Point, Rectangle, Sprite, Shape, Color};
use crate::{assets::*,WINDOW_WIDTH,WINDOW_HEIGHT};
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
    const PLAYER_GRAVITY: f32 = 3.0;
    const SPEED: f32 = 200.0;
    const JUMP: f32 = 10.0;

    pub fn new(position: Rectangle<f32>) -> Self {
        Player {
            position,
            velocity: (Self::SPEED,Self::JUMP),
            on_ground: true,
        }
    }

    pub fn update(&mut self, action: Action, seconds: f32) {
        match action {
            Action::Left => {
                self.position.x -= seconds * self.velocity.0;
                self.position.x = f32::clamp(self.position.x, 0.0, crate::WINDOW_WIDTH);
            }
            Action::Right => {
                self.position.x += seconds * self.velocity.0;
                self.position.x = f32::clamp(self.position.x, 0.0, crate::WINDOW_WIDTH);
            }
            Action::Up => {
                if self.on_ground {
                    self.velocity.1 = Self::JUMP;
                    self.on_ground = false;
                }
            }
            _ => (),
        }

        if !self.on_ground {
            self.velocity.1 += Self::PLAYER_GRAVITY * seconds;
    
            self.position.y += self.velocity.1 * seconds;
            self.position.y = f32::clamp(self.position.y, 0.0, WINDOW_HEIGHT)
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
        self.velocity   = (Player::SPEED,Player::JUMP); 
        self.on_ground = false;
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
    const GROUND_HEIGHT: f32 = 30.0;

    pub fn new(position: Rectangle<f32>) -> Self {
        Platform{ position }
    }

    pub fn ground() -> Self {
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
    const BUBBLE_STROKE_WIDTH: f32 = 2.0;
    const ENERGY_LOSS:f32 = 0.9;
    const BUBBLE_GRAVITY:f32 = 100.0;

    pub fn new(position: Rectangle<f32>, velocity: (f32,f32)) -> Self {
        let radius = position.width / 2.0;
        Bubble{ position, radius, velocity }
    }

    pub fn update(&mut self, seconds: f32) {
        self.velocity.1 += Self::BUBBLE_GRAVITY * seconds;

        self.position.x += self.velocity.0 * seconds;
        self.position.y += self.velocity.1 * seconds;
    }

    pub fn pop(&self) -> Vec<Self> {
        let mut position = self.position;
        position.width /= 2.0;
        position.height /= 2.0;

        vec![
            Bubble::new(position,(self.velocity.0,self.velocity.1)),
            Bubble::new(position,(-self.velocity.0,self.velocity.1)),
            ]
    }

    pub fn bounce_off_wall(&mut self) {
        self.velocity.0 = -self.velocity.0;
    }

    pub fn bounce_off_ceiling(&mut self) {
        self.velocity.1 = -self.velocity.1;
    }

    pub fn bounce_off_ground(&mut self) {
        self.velocity.1 *= -Self::ENERGY_LOSS;
    }

    pub fn bounce_off_platform_horizontal(&mut self) {
        self.velocity.0 *= -1.0;
    }

    pub fn bounce_off_platform_vertical(&mut self) {
        self.velocity.1 *= -1.0;
    }

    pub fn draw(&self, frame: &mut Frame, color: Color) {
        let mut shape = Mesh::new();
        let center = Point::new(self.position.center().x,self.position.center().y);
        shape.stroke(Shape::Circle {center,radius:self.radius}, color,Self::BUBBLE_STROKE_WIDTH);
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

    pub fn new(position: Rectangle<f32>, state: HarpoonState) -> Self {
        Harpoon { position ,state }
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
            self.position.y -= Self::HARPOON_VELOCITY * seconds;
            self.position.height += Self::HARPOON_VELOCITY * seconds;
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