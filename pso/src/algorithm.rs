use crate::{Food, Population};

use gif::{Encoder, Frame, Repeat};
use image::{Rgb, RgbImage};
use simple_logger::SimpleLogger;
use std::fs::File;

use crate::Config;

pub struct Algorithm {
    food: Food,
    config: Config,
    population: Population,
}

impl Algorithm {
    pub fn new(config: Config) -> Self {
        let _ = SimpleLogger::new().init();
        if config.output.with_image && !std::path::Path::new("cache").exists() {
            std::fs::create_dir_all("cache").unwrap();
        }

        let mut rng = rand::thread_rng();
        let food = Food::new(&mut rng, &config.food);
        let population = Population::new(&mut rng, &config.particle, &config.population);

        Self {
            food,
            config,
            population,
        }
    }

    pub fn update(&mut self) {
        self.food.update(&self.config.food);
        self.population
            .update(&self.food, &self.config.pso, &self.config.particle);
    }

    pub fn draw(&mut self, epoch: usize) {
        let mut image = RgbImage::new(800, 600);
        image.pixels_mut().for_each(|pixel| {
            *pixel = Rgb([253, 240, 213]);
        });

        self.food.draw(&mut image);
        self.population.draw(&mut image);
        image.save(format!("cache/{}.png", epoch)).unwrap();
    }

    pub fn save(&self) {
        let mut gif = File::create("result.gif").unwrap();
        let mut encoder = Encoder::new(&mut gif, 800, 600, &[]).unwrap();
        encoder.set_repeat(Repeat::Finite(1)).unwrap();
        (0..self.config.epochs).for_each(|i| {
            let mut image = image::open(format!("cache/{}.png", i))
                .unwrap()
                .to_rgba8()
                .into_raw();
            let frame = Frame::from_rgba(800, 600, &mut image);
            encoder.write_frame(&frame).unwrap();
        });
    }

    pub fn run(&mut self) {
        (0..self.config.epochs).for_each(|i| {
            if self.config.output.with_image {
                self.draw(i);
            }
            log::info!(
                "epoch={} - progress={:.2}%",
                i,
                i as f64 / self.config.epochs as f64 * 100.0
            );
            self.update();
        });
        if self.config.output.with_image {
            self.save();
        }
    }
}
