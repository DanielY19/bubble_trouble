use bubble_trouble::drawing_utils::PlayerSpriteSlices;
use bubble_trouble::entities::Player;
use coffee::graphics::{Color, Frame, Rectangle, Window, WindowSettings, Sprite, Point};
use coffee::load::Task;
use coffee::{Game, Result, Timer};

use bubble_trouble::{entities,assets,drawing_utils,WINDOW_HEIGHT,WINDOW_WIDTH};

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
    assets: assets::Assets,
    player: Player,
    player_draw_utils: PlayerSpriteSlices
}

impl GameState{
    pub fn load() -> Task<GameState>{
        assets::Assets::load().map(|assets| {
            let dimensions = assets.player_sprite_dimensions;
            let player_draw_utils = PlayerSpriteSlices::new(dimensions);

            let position = Rectangle{
                x:600.0,
                y:600.0,
                width:dimensions.0 as f32,
                height:dimensions.1 as f32,
            };

            let player = Player::new(position);
            GameState { assets, player, player_draw_utils }
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
        self.player.draw(frame, &self.assets,&self.player_draw_utils);
    }
}