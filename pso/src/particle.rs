use std::array;

use crate::Food;

use rand::{Rng, RngCore};

use image::{Rgb, RgbImage};
use imageproc::drawing::draw_filled_circle_mut;

use crate::{ConfigPSO, ConfigParticle};

#[derive(Clone)]
pub struct Particle {
    pub id: usize,
    pub velocity: [f64; 2],
    pub position: [f64; 2],
}

impl Particle {
    pub fn new(id: usize, rng: &mut dyn RngCore, config: &ConfigParticle) -> Self {
        let velocity = array::from_fn(|_| rng.gen_range(config.velocity[0]..config.velocity[1]));
        let position = array::from_fn(|_| rng.gen_range(config.position[0]..config.position[1]));
        Self {
            id,
            position,
            velocity,
        }
    }

    pub fn separation(&self, particles: &Vec<&Particle>) -> [f64; 2] {
        let position = self.position;
        let size = particles.len() as f64;
        let others = particles
            .iter()
            .map(|p| array::from_fn(|i| position[i] - p.position[i]))
            .collect::<Vec<[f64; 2]>>();

        array::from_fn(|i| -others.iter().map(|other| other[i]).sum::<f64>() / size)
    }

    pub fn alignment(&self, particles: &Vec<&Particle>) -> [f64; 2] {
        let size = particles.len();
        let mut result = [0.0; 2];
        particles.iter().for_each(|other| {
            result[0] += other.position[0];
            result[1] += other.position[1];
        });
        array::from_fn(|i| result[i] / size as f64)
    }

    pub fn cohesion(&self, particles: &Vec<&Particle>) -> [f64; 2] {
        let alignment = self.alignment(particles);
        [
            alignment[0] - self.position[0],
            alignment[1] - self.position[1],
        ]
    }

    pub fn update(
        &mut self,
        food: &Food,
        config: &ConfigPSO,
        particles: Vec<&Particle>,
        config_particle: &ConfigParticle,
    ) {
        let cohesion = self.cohesion(&particles);
        let alignment = self.alignment(&particles);
        let separation = self.separation(&particles);

        self.velocity = array::from_fn(|i| {
            let v = self.velocity[i]
                + config.cognitive * cohesion[i]
                + config.social * alignment[i]
                + config.inertia * separation[i]
                + (food.position[i] - self.position[i]);
            v.max(config_particle.velocity[0])
                .min(config_particle.velocity[1])
        });
        self.position =
            array::from_fn(|i| (self.position[i] + self.velocity[i]) % config_particle.position[1]);
    }

    pub fn draw(&self, image: &mut RgbImage, color: Rgb<u8>) {
        let radius = 5;

        let (width, height) = image.dimensions();
        let (width, height) = ((width - 100) as f64, (height - 100) as f64);
        let center = (
            (self.position[0] * width) as i32,
            (self.position[1] * height) as i32,
        );
        draw_filled_circle_mut(image, center, radius, color);
    }
}
