use coffee::graphics::{Rectangle,Color};
use crate::WINDOW_WIDTH;

use rand::{rng, Rng};
use rand::rngs::ThreadRng;

pub struct ParameterGenerator{
    generator: ThreadRng
}

impl ParameterGenerator {
    const PLATFORM_HEIGHT:f32 = 20.0;
    const PLATFORM_WALL_BOUND:f32 = 75.0;
    const PLATFORM_WIDTH_LOWER_BOUND:f32 = 200.0;
    const PLATFORM_WIDTH_HIGHER_BOUND:f32 = 300.0;

    pub fn new() -> ParameterGenerator {
        ParameterGenerator { generator: rng() }
    }

    pub fn generate_platform(&mut self, elevation: f32) -> Rectangle<f32> {
        let width:f32 = self.generator.random_range(ParameterGenerator::PLATFORM_WIDTH_LOWER_BOUND .. ParameterGenerator::PLATFORM_WIDTH_HIGHER_BOUND);

        let x = self.generator.random_range(ParameterGenerator::PLATFORM_WALL_BOUND ..= WINDOW_WIDTH - width - ParameterGenerator::PLATFORM_WALL_BOUND);
        //let y = self.generator.random_range(range); fixed elevation seems a better option

        Rectangle { x,y:elevation,width,height:ParameterGenerator::PLATFORM_HEIGHT }
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