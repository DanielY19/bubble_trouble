use bubble_trouble::entities::Player;
use coffee::graphics::{Color, Frame, Rectangle, Window, WindowSettings, Sprite, Point};
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
}

impl GameState{
    pub fn load() -> Task<GameState>{
        Assets::load().map(|assets| {
            let position = Rectangle{
                x:500.0,
                y:300.0,
                width:assets.player_sprite_slices.Idle.width as f32,
                height:assets.player_sprite_slices.Idle.height as f32,
            };

            let player = Player::new(position);
            GameState { assets, player }
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
        frame.clear(Color::new(0.5,0.5,0.5,0.0));
        self.player.draw(frame, &self.assets,self.assets.player_sprite_slices.Right);
    }
}