#[derive(Clone)]
pub struct CelestialObject {
    pub name: String,
    pub mass: f64,
    pub position: (f64, f64), // x, y coordinates
    pub velocity: (f64, f64), // x, y components
}

impl CelestialObject {
    pub fn new(name: String, mass: f64, position: (f64, f64), velocity: (f64, f64)) -> Self {
        CelestialObject {
            name,
            mass,
            position,
            velocity,
        }
    }
}

impl PartialEq for CelestialObject {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.mass == other.mass && self.position == other.position && self.velocity == other.velocity
    }
}