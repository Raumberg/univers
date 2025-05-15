#[allow(dead_code)]

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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CelestialType {
    Planet,
    Star,
    Particle,
}

impl Default for CelestialType {
    fn default() -> Self {
        CelestialType::Planet
    }
}

#[derive(Clone, Debug, Default)]
pub struct CelestialObject {
    pub name: String,
    pub mass: Mass,
    pub position: Position,  // x, y coordinates
    pub velocity: Velocity, // x, y components
    pub acceleration: Acceleration,
    pub prevposition: Position,
    pub kind: CelestialType,
    pub lifetime: Option<u32>, // Только для частиц
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
            prevposition,
            kind: CelestialType::Planet,
            lifetime: None,
        }
    }
    pub fn new_particle(
        position: Position,
        velocity: Velocity,
        mass: Mass,
        lifetime: u32,
    ) -> Self {
        CelestialObject {
            name: "Particle".to_string(),
            mass,
            position,
            velocity,
            acceleration: Velocity::new(0.0, 0.0),
            prevposition: position,
            kind: CelestialType::Particle,
            lifetime: Some(lifetime),
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

    pub fn radius(&self) -> f64 {
        match self.kind {
            CelestialType::Planet | CelestialType::Star => {
                // r = (3M / 4πρ)^(1/3), ρ ~ 5500 кг/м³
                let density = 5500.0;
                ((3.0 * self.mass) / (4.0 * std::f64::consts::PI * density)).cbrt()
            }
            CelestialType::Particle => 1e8, // частицы — маленькие
        }
    }
}

impl PartialEq for CelestialObject {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
        && self.mass == other.mass
        && self.position == other.position
        && self.velocity == other.velocity
        && self.kind == other.kind
    }
}