use crate::objects::{CelestialObject, G, Force, Acceleration};
use crate::vectors::Vec2D;

fn calculate_forces(bodies: &Vec<CelestialObject>) -> Vec<Force> {
    let mut forces = vec![(0.,0.); bodies.len()];
    for i in 0..bodies.len() - 1 {
        for j in i + 1..bodies.len() {
            let f = bodies[i].get_force(&bodies[j]);
            forces[i].0 += f.0;
            forces[i].1 += f.1;
            forces[j].0 += -f.0;
            forces[j].1 += -f.1;
        }
    }
    forces
}

fn calculate_accels(bodies: &Vec<CelestialObject>, forces: Vec<Force>) -> Vec<Acceleration> {
    let mut accels = vec![(0.,0.); bodies.len()];
    for i in 0..bodies.len() {
        let ax = forces[i].0 / bodies[i].mass;
        let ay = forces[i].1 / bodies[i].mass;
        accels[i] = (ax, ay);
    }
    accels
}

fn calculate_gravitational_force(body1: &CelestialObject, body2: &CelestialObject) -> (f64, f64) {
    // Calculate the distance squared between the two bodies
    let distance = body1.position - body2.position;

    // Calculate the force magnitude
    let force_magnitude = G * body1.mass * body2.mass / distance.norm();

    // Calculate the force components
    let force_direction = distance / distance.norm();

    force_direction * force_magnitude
}

fn update_body(body: &mut CelestialObject, bodies: &[CelestialObject], dt: f64) {
    // Initialize the force components to 0
    let mut force = Vec2D::zeros();

    // Calculate the total force on the body
    for other_body in bodies {
        if body != other_body {
            force += calculate_gravitational_force(body, other_body);
        }
    }
    body.acceleration = force / body.mass;
    body.velocity += body.acceleration * dt;
    body.position += body.velocity * dt;
}

pub fn simulate(bodies: &mut [CelestialObject], dt: f64, num_steps: usize) {
    // Check if num_steps is 0
    if num_steps == 0 {
        return;
    }

    // Simulate the bodies
    for _ in 0..num_steps {
        for body in bodies.iter_mut() {
            update_body(body, bodies, dt);
        }
    }
}