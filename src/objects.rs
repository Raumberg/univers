use crate::vectors::Vec2D;

pub type Velocity = Vec2D<f64>;
pub type Position = Vec2D<f64>;
pub type Mass = f64;
pub type Acceleration = Vec2D<f64>;
pub type Distance = Vec2D<f64>;
pub type Force = Vec2D<f64>;

pub const G: f64 = 6.67428e-11; // gravitational constant, in m^3 kg^-1 s^-2
pub const AU: f64 = 1.4960e+11; // astronomical units, ~distance between sun and earth

pub trait Sim {
    fn get_name(&self) -> String;
    fn get_position(&self) -> Position;     // Vector2<T>
    fn get_velocity(&self) -> Velocity;     // Vector2<T>
    fn get_mass(&self) -> f64; 
    // fn simulate(&mut self)
}

#[derive(Clone, Debug)]
pub struct CelestialObject {
    pub name: String,
    pub mass: f64,
    pub position: Vec2D<f64>, // x, y coordinates
    pub velocity: Vec2D<f64>, // x, y components
    pub acceleration: Vec2D<f64>
}

impl CelestialObject {
    pub fn new(name: String, mass: f64, position: Vec2D<f64>, velocity: Vec2D<f64>, acceleration: Vec2D<f64>) -> Self {
        CelestialObject {
            name,
            mass,
            position,
            velocity,
            acceleration
        }
    }
    pub fn get_distance(&self, other: &Position) -> Distance {
        let dx: f64 = other.0 - self.position.0;
        let dy: f64 = other.1 - self.position.1;
        (dx, dy)
    }

    pub fn get_force(&self, other: &CelestialObject) -> Force {
        let dist = self.get_distance(&other.position);
        if dist.0 == 0.0 && dist.1 == 0.0 {
            return (0.0, 0.0);
        }
        let r = (dist.0.powi(2) + dist.1.powi(2)).sqrt();
        let cos = dist.0 / r;
        let sin = dist.1 / r;
        let f = (G * self.mass * other.mass) / r.powi(2);
        let fx = f * cos;
        let fy = f * sin;
        (fx, fy)
    } 
}

impl PartialEq for CelestialObject {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.mass == other.mass && self.position == other.position && self.velocity == other.velocity
    }
}