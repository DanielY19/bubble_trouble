use bubble_trouble::entities::Player;
use coffee::graphics::{Color, Frame, Shape, Rectangle, Window, WindowSettings, Mesh};
use coffee::load::Task;
use coffee::{Game, Result, Timer};
use rand::{rng, Rng};
use rand::rngs::ThreadRng;

use bubble_trouble::{WINDOW_HEIGHT,WINDOW_WIDTH};
use bubble_trouble::assets::*;
use bubble_trouble::entities::*;

pub struct ParameterGenerator{
    generator: ThreadRng
}

impl ParameterGenerator {
    const PLATFORM_HEIGHT:f32 = 20.0;
    const PLATFORM_WIDTH_LOWER_BOUND:f32 = 100.0;
    const PLATFORM_WIDTH_HIGHER_BOUND:f32 = 200.0;

    pub fn new() -> ParameterGenerator {
        ParameterGenerator { generator: rng() }
    }

    pub fn generate_platform(&mut self) -> Rectangle<f32> {
        let x = self.generator.random_range(0.0 ..= WINDOW_WIDTH);
        let y = self.generator.random_range(0.0 ..= WINDOW_HEIGHT);
        let width:f32 = self.generator.random_range(Self::PLATFORM_WIDTH_LOWER_BOUND .. Self::PLATFORM_WIDTH_HIGHER_BOUND);
        let height:f32 = Self::PLATFORM_HEIGHT;

        Rectangle { x,y,width,height }
    }

    pub fn generate_random_color(&mut self) -> Color {
        Color{
            r: self.generator.random_range(0.0..=1.0),
            g: self.generator.random_range(0.0..=1.0),
            b: self.generator.random_range(0.0..=1.0),
            a:1.0
        }
    }
}

pub fn is_debug_active() -> bool {
    std::env::var("DEBUG").is_ok()
}

fn main() -> Result<()> {
    GameState::run(WindowSettings {
        title: String::from("bubble_trouble"),
        size: (WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32),
        resizable: true,
        fullscreen: false,
        maximized: false,
    })
}

struct GameState {
    assets: Assets,
    player: Player,
    harpoon: Harpoon,
    platforms: Vec<Platform>,
    bubbles:Vec<Bubble>,

    parameter_generator: ParameterGenerator,
}

impl GameState{
    const SECONDS_FOR_FRAME: f32 = 1.0 / Self::TICKS_PER_SECOND as f32;

    pub fn load() -> Task<GameState>{
        Assets::load().map(|assets| {
            let number_of_platforms = 4;
            let mut parameter_generator = ParameterGenerator::new();

            let ground = Platform::ground();
            let ground_y_pos = ground.position.y;
            let mut platforms = vec![ground];

            for _ in 1..number_of_platforms {
                platforms.push(Platform::new(parameter_generator.generate_platform()));
            }

            let bubble = Bubble::new(Rectangle { x: 500.0, y: 300.0, width: 50.0, height: 50.0 },(150.0,75.0));
            let bubbles = vec![bubble];

            let player_start_position = Rectangle{
                x:WINDOW_WIDTH / 2.0 - assets.player_sprite_slices.idle.width as f32,
                y:ground_y_pos - assets.player_sprite_slices.idle.height as f32,
                width:assets.player_sprite_slices.idle.width as f32,
                height:assets.player_sprite_slices.idle.height as f32,
            };

            let harpoon_start_position = Rectangle {
                x:player_start_position.x,
                y:300.0,
                width:assets.harpoon_sprite_sheet.width() as f32,
                height: 0.0,
            };

            let harpoon = Harpoon::new(harpoon_start_position, 100.0, HarpoonState::Active);

            let player = Player::new(player_start_position);
            GameState { assets, player, harpoon ,platforms,bubbles, parameter_generator }
        })
    }
}

impl Game for GameState {
    type Input = ();
    type LoadingScreen = ();

    fn load(window: &Window) -> Task<GameState> {
        GameState::load()
    }

    fn update(&mut self, _window: &Window) {
        self.harpoon.update(Self::SECONDS_FOR_FRAME);

        for bubble in &mut self.bubbles{
            bubble.update(Self::SECONDS_FOR_FRAME);

            for platform in &self.platforms[1..] {
                bubble.bounce_off_platform(platform);
            }
        }
        
        if self.bubbles.len() < 2 {
            let temp = self.bubbles.remove(0);
            self.bubbles.append(&mut temp.pop());
        }
    }

    fn draw(&mut self, frame: &mut Frame, _timer: &Timer) {
        frame.clear(Assets::GREY);
        self.player.draw(frame, &self.assets,self.assets.player_sprite_slices.idle);
        self.harpoon.draw(frame, &self.assets);
        let mut shapes = vec![self.player.position,self.harpoon.position];
        
        for platform in &self.platforms {
            platform.draw(frame);
        }
        
        for bubble in &self.bubbles {
            bubble.draw(frame,self.parameter_generator.generate_random_color());
            shapes.push(bubble.position);
        }
        
        if is_debug_active(){
            for shape in &shapes {
                let mut mesh=  Mesh::new();
                mesh.stroke(Shape::Rectangle(*shape),Assets::BLACK, 2.0);
                mesh.draw(&mut frame.as_target());
            }    
        }
    }
}