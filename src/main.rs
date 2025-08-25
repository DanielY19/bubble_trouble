use bubble_trouble::entities::Player;
use coffee::graphics::{Frame, Rectangle, Window, WindowSettings, Point};
use coffee::load::Task;
use coffee::{Game, Result, Timer};

use bubble_trouble::{WINDOW_HEIGHT,WINDOW_WIDTH};
use bubble_trouble::assets::*;
use bubble_trouble::entities::*;

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
    platforms: Vec<Platform>
}

impl GameState{
    pub fn load() -> Task<GameState>{
        Assets::load().map(|assets| {
            let number_of_platforms = 4;
            let mut platform_generator = PlatformGenerator::new();

            let ground = Platform::ground();
            let ground_y_pos = ground.position.y;
            let mut platforms = vec![ground];

            for _ in 1..number_of_platforms {
                platforms.push(Platform::new(platform_generator.generate_platform()));
            }

            let start_position = Rectangle{
                x:WINDOW_WIDTH / 2.0 - assets.player_sprite_slices.idle.width as f32,
                y:ground_y_pos - assets.player_sprite_slices.idle.height as f32,
                width:assets.player_sprite_slices.idle.width as f32,
                height:assets.player_sprite_slices.idle.height as f32,
            };


            let player = Player::new(start_position);
            GameState { assets, player, platforms }
        })
    }
}

impl Game for GameState {
    type Input = ();
    type LoadingScreen = ();

    fn load(window: &Window) -> Task<GameState> {
        GameState::load()
    }

    fn draw(&mut self, frame: &mut Frame, _timer: &Timer) {
        frame.clear(Assets::GREY);
        self.player.draw(frame, &self.assets,self.assets.player_sprite_slices.idle);
        
        for platform in &mut self.platforms {
            platform.draw(frame);
        }
    }
}