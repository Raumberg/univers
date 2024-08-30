use crate::space::objects::CelestialObject;
use crate::engine::physics::{QuadTree, Rectangle};
use crate::space::types::*;

pub struct Simulation {
    pub bodies: Vec<CelestialObject>,
    pub quad_tree: QuadTree,
    pub theta: f64,
    pub time_step: f64,
}

impl Simulation {
    pub fn new(bodies: Vec<CelestialObject>, theta: f64, time_step: f64) -> Self {
        let bounds = Rectangle::new(-1000.0, -1000.0, 2000.0, 2000.0);
        let quad_tree = QuadTree::new(bounds, 4);
        Simulation { bodies, quad_tree, theta, time_step }
    }

    pub fn step(&mut self) {
        self.quad_tree = QuadTree::new(self.quad_tree.bounds, 4);
        for body in &self.bodies {
            self.quad_tree.insert(body.clone());
        }

        for body in &mut self.bodies {
            let force = self.quad_tree.traverse(&body, self.theta);
            let acceleration = Acceleration::new(force.x / body.mass, force.y / body.mass);
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