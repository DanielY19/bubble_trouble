use coffee::graphics::{Frame, Shape, Rectangle, Window, WindowSettings, Mesh};
use coffee::{Game, Result, Timer};
use coffee::load::Task;
use coffee::input::{Keyboard,keyboard::KeyCode};

use bubble_trouble::{WINDOW_HEIGHT,WINDOW_WIDTH,assets::*,entities::*,collision::*,parameter_generator::ParameterGenerator};

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
    const SECONDS_FOR_FRAME: f32 = 1.0 / GameState::TICKS_PER_SECOND as f32;

    pub fn load() -> Task<GameState>{
        Assets::load().map(|assets| {
            let mut parameter_generator = ParameterGenerator::new();

            let platforms = Platform::generate_platforms(&mut parameter_generator);
            let ground_y_pos = platforms.first().unwrap().position.y;

            let bubble = Bubble::new(Rectangle { x: 500.0, y: 300.0, width: 50.0, height: 50.0 },25.0,(150.0,75.0));
            let bubbles = vec![bubble];

            let player_start_position = Rectangle{
                x:WINDOW_WIDTH / 2.0 - assets.player_sprite_slices.idle.width as f32 / 2.0,
                y:ground_y_pos - assets.player_sprite_slices.idle.height as f32,
                width:assets.player_sprite_slices.idle.width as f32,
                height:assets.player_sprite_slices.idle.height as f32,
            };

            let harpoon_start_position = Rectangle {
                x:player_start_position.center().x - assets.harpoon_sprite_sheet.width() as f32 / 2.0,
                y:ground_y_pos,
                width:assets.harpoon_sprite_sheet.width() as f32,
                height: 0.0,
            };

            let harpoon = Harpoon::new(harpoon_start_position, HarpoonState::Inactive);

            let mut player = Player::new(player_start_position,&assets.player_sprite_slices.idle);
            player.handle_input(Action::Idle);

            GameState { assets, player, harpoon ,platforms,bubbles, parameter_generator }
        })
    }
}

impl<'a> Game for GameState {
    type Input = Keyboard;
    type LoadingScreen = ();

    fn load(window: &Window) -> Task<GameState> {
        GameState::load()
    }

    fn update(&mut self, _window: &Window) {
        CollisionSystem::player_platforms_collision(&mut self.player, &self.platforms);
        CollisionSystem::bubbles_walls_collision(&mut self.bubbles);
        CollisionSystem::bubbles_ceiling_collision(&mut self.bubbles);
        CollisionSystem::bubbles_platforms_collision(&mut self.bubbles, &self.platforms);
        CollisionSystem::harpoon_platform_collision(&mut self.harpoon, &self.platforms);
        CollisionSystem::harpoon_bubbles_collision(&mut self.harpoon, &mut self.bubbles);
        CollisionSystem::harpoon_ceiling_collision(&mut self.harpoon);

        self.player.update( GameState::SECONDS_FOR_FRAME);
        self.harpoon.update(GameState::SECONDS_FOR_FRAME);

        for bubble in &mut self.bubbles{
            bubble.update(GameState::SECONDS_FOR_FRAME);
        }
    }

    fn draw(&mut self, frame: &mut Frame, _timer: &Timer) {
        frame.clear(Assets::GREY);
        self.player.draw(frame, &self.assets);
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
                mesh.stroke(Shape::Rectangle(*shape),Assets::BLACK, Assets::MESH_STROKE_WIDTH);
                mesh.draw(&mut frame.as_target());
            }    
        }
    }

    fn is_finished(&self) -> bool {
        CollisionSystem::player_bubbles_collision(&self.player, &self.bubbles)
    }

    fn interact(&mut self, _input: &mut Self::Input, _window: &mut Window) {
        if _input.is_key_pressed(KeyCode::W) {
            self.player.handle_input(Action::Up);
            self.player.motion = self.assets.player_sprite_slices.idle.clone();
        }
        if _input.is_key_pressed(KeyCode::A) {
            self.player.handle_input(Action::Left);
            self.player.animate(GameState::SECONDS_FOR_FRAME, 
                &self.assets.player_sprite_slices.left,
                &self.assets.player_sprite_slices.left_step);
        }
        if _input.is_key_pressed(KeyCode::D) {
            self.player.handle_input(Action::Right);
            self.player.animate(GameState::SECONDS_FOR_FRAME, 
                &self.assets.player_sprite_slices.right,
                &self.assets.player_sprite_slices.right_step);
        }
        if _input.is_key_pressed(KeyCode::Space) {
            self.player.motion = self.assets.player_sprite_slices.idle.clone();
            if self.player.on_ground {
                self.harpoon.fire(&self.player.position);
            }
        }
        else if _input.was_key_released(KeyCode::W) 
        || _input.was_key_released(KeyCode::A)
        || _input.was_key_released(KeyCode::D)  {
            self.player.handle_input(Action::Idle);
            self.player.motion = self.assets.player_sprite_slices.idle.clone();
        }
    }
}