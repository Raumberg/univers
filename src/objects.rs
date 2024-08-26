use nalgebra::{Point2, Vector2};

pub type Velocity = Vector2<f64>;
pub type Position = Point2<f64>;
pub type Mass = f64;
pub type Acceleration = Vector2<f64>;
pub type Distance = Vector2<f64>;
pub type Force = Vector2<f64>;

pub const G: f64 = 6.67428e-11; // gravitational constant, in m^3 kg^-1 s^-2
pub const AU: f64 = 1.4960e+11; // astronomical units, ~distance between sun and earth

pub trait Sim {
    fn get_name(&self) -> String;
    fn get_position(&self) -> Position; // Point2<T>
    fn get_velocity(&self) -> Velocity; // Vector2<T>
    fn get_mass(&self) -> f64;
    // fn simulate(&mut self)
}

#[derive(Clone, Debug)]
pub struct CelestialObject {
    pub name: String,
    pub mass: f64,
    pub position: Point2<f64>,  // x, y coordinates
    pub velocity: Vector2<f64>, // x, y components
    pub acceleration: Vector2<f64>,
    pub prevposition: Point2<f64>,
}

impl CelestialObject {
    pub fn new(
        name: String,
        mass: f64,
        position: Point2<f64>,
        velocity: Vector2<f64>,
        acceleration: Vector2<f64>,
        prevposition: Point2<f64>,
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
    pub fn get_distance(&self, other: &Position) -> Vector2<f64> {
        other - self.position
    }

    pub fn get_force(&self, other: &CelestialObject) -> Vector2<f64> {
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
