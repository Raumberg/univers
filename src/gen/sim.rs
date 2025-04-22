use crate::space::objects::{Acceleration, CelestialObject};
use nalgebra::Vector2;

pub struct Simulation {
    pub bodies: Vec<CelestialObject>,
    pub time_step: f64,
}

impl Simulation {
    pub fn new(bodies: Vec<CelestialObject>, time_step: f64) -> Self {
        Simulation { bodies, time_step }
    }

    pub fn step(&mut self) {
        // Calculate forces for each body
        let body_count = self.bodies.len();
        let mut forces = vec![Vector2::new(0.0, 0.0); body_count];
        
        // Store current positions to avoid borrowing issues
        let positions: Vec<(&str, nalgebra::Point2<f64>, f64)> = self.bodies
            .iter()
            .map(|b| (b.name.as_str(), b.position, b.mass))
            .collect();
        
        // Calculate forces between all pairs of bodies
        for i in 0..body_count {
            for j in 0..body_count {
                if i != j {
                    let (_, pos_i, mass_i) = positions[i];
                    let (_, pos_j, mass_j) = positions[j];
                    
                    let direction = pos_j - pos_i;
                    let distance_squared = direction.norm_squared();
                    
                    if distance_squared > 0.0 {
                        let magnitude = (crate::space::objects::G * mass_i * mass_j) / distance_squared;
                        forces[i] += magnitude * direction.normalize();
                    }
                }
            }
        }
        
        // Update positions based on forces
        for (i, body) in self.bodies.iter_mut().enumerate() {
            let acceleration = Acceleration::new(forces[i].x / body.mass, forces[i].y / body.mass);
            body.velocity += acceleration * self.time_step;
            body.position += body.velocity * self.time_step;
        }
    }

    pub fn run(&mut self, iterations: usize, printable: bool) {
        let mut iter = 0;
        while iter < iterations {
            self.step();
            iter += 1;
            if printable && iter % 10 == 0 {
                self.state();
            }
        }
    }

    pub fn state(&self) {
        for (i, body) in self.bodies.iter().enumerate() {
            println!("Body {}: Position ({}, {}), Velocity ({}, {})", i, body.position.x, body.position.y, body.velocity.x, body.velocity.y);
        }
    }
}