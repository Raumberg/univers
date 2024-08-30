#[allow(dead_code)]

use nalgebra::{Point2, Vector2};

use crate::space::types::*;

pub enum CelestialBodyType {
    Star,
    Planet,
    Moon,
    Asteroid,
    Comet,
    GasGiant,
    System
}

#[derive(Clone, Debug, Default)]
pub struct CelestialObject {
    pub name: String,
    pub mass: Mass,
    pub position: Position,  // x, y coordinates
    pub velocity: Velocity, // x, y components
    pub acceleration: Acceleration,
    pub prevposition: Position,
}

impl CelestialObject {
    pub fn new(
        name: String,
        mass: Mass,
        position: Position,
        velocity: Velocity,
        acceleration: Acceleration,
        prevposition: Position,
    ) -> Self {
        CelestialObject {
            name,
            mass,
            position,
            velocity,
            acceleration,
            prevposition: position,
        }
    }
    pub fn get_distance(&self, other: &Position) -> Distance {
        other - self.position
    }

    pub fn get_force(&self, other: &CelestialObject) -> Force {
        let dist = self.get_distance(&other.position);
        if dist.norm() == 0.0 {
            return Vector2::new(0.0, 0.0);
        }
        let f = (G * self.mass * other.mass) / dist.norm_squared();
        f * dist.normalize()
    }
}

impl PartialEq for CelestialObject {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
        && self.mass == other.mass
        && self.position == other.position
        && self.velocity == other.velocity
    }
}

// impl Clone for CelestialObject {
//     fn clone(&self) -> Self {
//         CelestialObject {
//             name: self.name.clone(),
//             mass: self.mass,
//             position: self.position.clone(),
//             velocity: self.velocity.clone(),
//             acceleration: self.acceleration.clone(),
//             prevposition: self.prevposition.clone(),
//         }
//     }
// }