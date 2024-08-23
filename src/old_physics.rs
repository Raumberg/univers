use crate::objects::CelestialObject;
use crate::system::SolarSystem;

const G: f64 = 6.67430e-11; // gravitational constant in m^3 kg^-1 s^-2

fn calculate_gravitational_force(body1: &CelestialObject, body2: &CelestialObject) -> (f64, f64) {
    let distance_squared = (body2.position.0 - body1.position.0).powi(2) + (body2.position.1 - body1.position.1).powi(2);
    let force_magnitude = G * body1.mass * body2.mass / distance_squared;
    let force_x = force_magnitude * (body2.position.0 - body1.position.0) / distance_squared.sqrt();
    let force_y = force_magnitude * (body2.position.1 - body1.position.1) / distance_squared.sqrt();
    (force_x, force_y)
}

fn update_body(body: &mut CelestialObject, bodies: &[CelestialObject], dt: f64) {
    let mut force_x = 0.0;
    let mut force_y = 0.0;
    for other_body in bodies {
        if body != other_body {
            let (fx, fy) = calculate_gravitational_force(body, other_body);
            force_x += fx;
            force_y += fy;
        }
    }
    let acceleration_x = force_x / body.mass;
    let acceleration_y = force_y / body.mass;
    body.velocity.0 += acceleration_x * dt;
    body.velocity.1 += acceleration_y * dt;
    body.position.0 += body.velocity.0 * dt;
    body.position.1 += body.velocity.1 * dt;
}

pub fn simulate(bodies: &mut [CelestialObject], dt: f64, num_steps: usize) {
    for _ in 0..bodies {
        for body in bodies.iter_mut() {
            update_body(body, bodies, dt);
        }
    }
}

pub trait Physics {
    fn simulate(&mut self, dt: f64);
}

pub struct Engine {
    pub solar_system: SolarSystem,
}

// Verlet Integration method for more accurate numerical integration
impl Physics for Engine {
    fn simulate(&mut self, dt: f64) {
        simulate(&mut self.solar_system.bodies, dt, 1); // assuming num_steps = 1

        // let bodies_clone = self.solar_system.bodies.clone();
        // for body in &mut self.solar_system.bodies {
        //     let mut acceleration = (0.0, 0.0);
        //     for other_body in &bodies_clone {
        //         if body != other_body {
        //             let distance_squared = (body.position.0 - other_body.position.0).powi(2) + (body.position.1 - other_body.position.1).powi(2);
        //             let force_magnitude = self.solar_system.g * body.mass * other_body.mass / distance_squared;
        //             let force_direction = ((other_body.position.0 - body.position.0) / distance_squared.sqrt(), (other_body.position.1 - body.position.1) / distance_squared.sqrt());
        //             acceleration.0 += force_direction.0 * force_magnitude / body.mass;
        //             acceleration.1 += force_direction.1 * force_magnitude / body.mass;
        //         }
        //     }
        //     let new_velocity_0 = body.velocity.0 + acceleration.0 * dt;
        //     let new_velocity_1 = body.velocity.1 + acceleration.1 * dt;
        //     let new_position_0 = body.position.0 + new_velocity_0 * dt;
        //     let new_position_1 = body.position.1 + new_velocity_1 * dt;
        //     body.velocity.0 = new_velocity_0;
        //     body.velocity.1 = new_velocity_1;
        //     body.position.0 = new_position_0;
        //     body.position.1 = new_position_1;
        // }
    }
}