use std::sync::{Arc, Mutex};
use crate::space::system::{StarSystem, Simulatable, SimulationAlgorithm};

#[derive(Clone)]
pub struct SimulationState {
    pub system: StarSystem,
    pub time_elapsed: f64,
    pub time_step: f64,
    pub running: bool,
    pub speed: usize,
    pub algorithm: SimulationAlgorithm,
}

impl SimulationState {
    pub fn new() -> Self {
        let system = StarSystem::solar();
        let algorithm = system.algorithm;
        SimulationState {
            system,
            time_elapsed: 0.0,
            time_step: 3600.0,
            running: true,
            speed: 5,
            algorithm,
        }
    }

    pub fn step(&mut self) {
        if self.running {
            self.system.simulate(self.time_step, self.speed);
            self.time_elapsed += self.time_step * self.speed as f64;
        }
    }

    pub fn reset(&mut self) {
        self.system = StarSystem::solar();
        self.time_elapsed = 0.0;
    }

    pub fn set_speed(&mut self, speed: usize) {
        self.speed = speed;
    }

    pub fn set_algorithm(&mut self, algo: SimulationAlgorithm) {
        self.system.set_algorithm(algo);
        self.algorithm = algo;
    }

    pub fn pause(&mut self) {
        self.running = false;
    }
    pub fn resume(&mut self) {
        self.running = true;
    }
} 