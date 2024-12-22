use std::array;

use image::{Rgb, RgbImage};
use imageproc::drawing::draw_filled_circle_mut;
use rand::{Rng, RngCore};

use crate::ConfigFood;

pub struct Food {
    pub position: [f64; 2],
    pub velocity: [f64; 2],
}

impl Food {
    pub fn new(rng: &mut dyn RngCore, config: &ConfigFood) -> Self {
        Self {
            position: array::from_fn(|_| rng.gen_range(config.position[0]..config.position[1])),
            velocity: array::from_fn(|_| rng.gen_range(config.velocity[0]..config.velocity[1])),
        }
    }

    pub fn update(&mut self, config: &ConfigFood) {
        self.position =
            array::from_fn(|i| (self.position[i] + self.velocity[i]) % config.position[1]);
    }

    pub fn draw(&self, image: &mut RgbImage) {
        let radius = 5;
        let color = Rgb([214, 40, 40]);
        let (width, height) = image.dimensions();
        let (width, height) = ((width - 100) as f64, (height - 100) as f64);
        let center = (
            (self.position[0] * width) as i32,
            (self.position[1] * height) as i32,
        );
        draw_filled_circle_mut(image, center, radius, color);
    }
}
