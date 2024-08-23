use crate::objects::CelestialObject;

// Define the solar system
pub struct SolarSystem {
    pub bodies: Vec<CelestialObject>,
    pub g: f64, // gravitational constant in m^3 kg^-1 s^-2
}

impl SolarSystem {
    // Constructor to create a new solar system
    pub fn new() -> Self {
        let sun = CelestialObject::new("Sun".to_string(), 1.989e30, (0.0, 0.0), (0.0, 0.0));
        let mercury = CelestialObject::new("Mercury".to_string(), 3.302e23, (57.909e9, 0.0), (0.0, 47.36e3));
        let venus = CelestialObject::new("Venus".to_string(), 4.869e24, (108.208e9, 0.0), (0.0, 35.02e3));
        let earth = CelestialObject::new("Earth".to_string(), 5.972e24, (149.596e9, 0.0), (0.0, 29.78e3));
        let moon = CelestialObject::new("Moon".to_string(), 7.349e22, (384.4e6, 0.0), (0.0, 1.022e3));
        let mars = CelestialObject::new("Mars".to_string(), 6.419e23, (227.939e9, 0.0), (0.0, 24.07e3));
        let jupiter = CelestialObject::new("Jupiter".to_string(), 1.898e27, (778.299e9, 0.0), (0.0, 13.07e3));
        let io = CelestialObject::new("Io".to_string(), 8.931e22, (426.0e6, 0.0), (0.0, 17.34e3));
        let europa = CelestialObject::new("Europa".to_string(), 4.879e22, (670.9e6, 0.0), (0.0, 13.86e3));
        let ganymede = CelestialObject::new("Ganymede".to_string(), 3.275e23, (1070.4e6, 0.0), (0.0, 10.87e3));
        let callisto = CelestialObject::new("Callisto".to_string(), 1.075e23, (1882.7e6, 0.0), (0.0, 8.22e3));

        let g = 6.67430e-11; 

        SolarSystem {
            bodies: vec![
                sun,
                mercury,
                venus,
                earth,
                moon,
                mars,
                jupiter,
                io,
                europa,
                ganymede,
                callisto,
            ],
            g
        }
    }

    pub fn simulate(&mut self, dt: f64, num_steps: usize) {
        crate::physics::simulate(&mut self.bodies, dt, num_steps);
    }

    pub fn run(&mut self, dt: f64) {
        self.simulate(dt, 100);
    }
}