use pso::{Algorithm, Config};

fn main() {
    let config = Config::new("config.json");
    let mut algorithm = Algorithm::new(config);
    algorithm.run();
}
