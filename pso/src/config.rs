use serde::Deserialize;

#[derive(Deserialize)]
pub struct ConfigPopulation {
    pub size: usize,
}

#[derive(Deserialize)]
pub struct ConfigOutput {
    pub with_image: bool,
}

#[derive(Deserialize)]
pub struct ConfigPSO {
    pub social: f64,
    pub inertia: f64,
    pub cognitive: f64,
}

#[derive(Deserialize)]
pub struct ConfigParticle {
    pub velocity: [f64; 2],
    pub position: [f64; 2],
}

#[derive(Deserialize)]
pub struct ConfigFood {
    pub position: [f64; 2],
    pub velocity: [f64; 2],
}

#[derive(Deserialize)]
pub struct Config {
    pub epochs: usize,
    pub pso: ConfigPSO,
    pub food: ConfigFood,
    pub output: ConfigOutput,
    pub particle: ConfigParticle,
    pub population: ConfigPopulation,
}

impl Config {
    pub fn new(path: &str) -> Self {
        let config = std::fs::read_to_string(path).unwrap();
        let config: Config = serde_json::from_str(&config).unwrap();
        config
    }
}
