use crate::{Food, Particle};

use image::{Rgb, RgbImage};
use rand::RngCore;

use crate::{ConfigPSO, ConfigParticle, ConfigPopulation};

pub struct Population {
    pub particles: Vec<Particle>,
}

impl Population {
    pub fn new(
        rng: &mut dyn RngCore,
        config_particle: &ConfigParticle,
        config_population: &ConfigPopulation,
    ) -> Self {
        Self {
            particles: (0..config_population.size)
                .map(|id| Particle::new(id, rng, config_particle))
                .collect(),
        }
    }

    pub fn update(&mut self, food: &Food, config: &ConfigPSO, config_particle: &ConfigParticle) {
        let particles: Vec<Particle> = self.particles.clone();
        self.particles.iter_mut().for_each(|particle| {
            let other: Vec<&Particle> = particles.iter().filter(|p| p.id != particle.id).collect();
            particle.update(food, config, other, config_particle);
        });
    }

    pub fn draw(&self, image: &mut RgbImage) {
        let color = Rgb([0, 48, 73]);

        self.particles.iter().for_each(|particle| {
            particle.draw(image, color);
        });
    }
}
