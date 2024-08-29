use nalgebra::{Point2, Vector2};

use crate::space::objects::CelestialObject;
use crate::engine::physics;

pub struct Star;
pub struct Planet;
pub struct Moon;

/// A trait for systems that can be simulated.
pub trait Simulatable {
    /// A function to get the bodies of the system
    fn bodies(&self) -> &Vec<CelestialObject>;
    /// A function to take control over the simulation by providing number of steps
    fn simulate(&mut self, dt: f64, num_steps: usize);

    /// Runs the star system simulation.
    ///
    /// This method simulates the star system for a specified number of time steps,
    /// using a variable time step to maintain a maximum acceleration tolerance.
    ///
    /// # Parameters
    ///
    /// * `max_acceleration_tolerance`: The maximum acceleration tolerance for the simulation.
    ///   A smaller value will result in a more accurate simulation, but may slow down the simulation.
    ///   A larger value will result in a faster simulation, but may sacrifice some accuracy.
    /// * `num_steps`: The number of time steps to simulate.
    ///   A smaller value will result in a shorter simulation, which may not capture the full dynamics of the system.
    ///   A larger value will result in a longer simulation, which may capture more of the system's behavior.
    ///
    /// # Example
    ///
    /// ```
    /// system.run(1.0, 1000);
    /// ```
    fn run(&mut self, max_acceleration_tolerance: f64, num_steps: usize) {
        let mut dt = 1.0; // initial time step
        let mut max_acceleration = 0.0; // initial maximum acceleration

        for _ in 0..num_steps {
            // Calculate the maximum acceleration of the bodies
            for body in self.bodies() {
                let acceleration = body.acceleration.norm();
                if acceleration > max_acceleration {
                    max_acceleration = acceleration;
                }
            }
            // Adjust the time step based on the maximum acceleration
            if max_acceleration > max_acceleration_tolerance {
                dt *= 0.5; // reduce the time step if the acceleration is too high
            } else {
                dt *= 1.1; // increase the time step if the acceleration is low
            }
            // Simulate the next time step
            self.simulate(dt, 1);
        }
    }

    /// A function to log the final state of the system.
    fn state(&self) {
        println!("State of the system:");
        for body in self.bodies() {
            println!("  {}:", body.name);
            println!("    Position: ({:.2}, {:.2})", body.position.x, body.position.y);
            println!("    Velocity: ({:.2}, {:.2})", body.velocity.x, body.velocity.y);
        }
    }
}

pub struct StarSystem {
    pub bodies: Vec<CelestialObject>,
    pub g: f64, // gravitational constant in m^3 kg^-1 s^-2
}

impl StarSystem {
    pub fn new() -> Self {
        StarSystem {
            bodies: Vec::new(),
            g: 6.67430e-11, 
        }
    }
    
    pub fn add_body(&mut self, body: CelestialObject) {
        self.bodies.push(body);
    }

    pub fn solar() -> Self {
        let sun = CelestialObject::new(
            "Sun".to_string(),
            1.989e30,
            Point2::new(0.0, 0.0),
            Vector2::new(0.0, 0.0),
            Vector2::new(0.0, 0.0),
            Point2::new(0.0, 0.0), // initial prevposition
        );
        let mercury = CelestialObject::new(
            "Mercury".to_string(),
            3.302e23,
            Point2::new(57.909e9, 0.0),
            Vector2::new(0.0, 47.36e3),
            Vector2::new(0.0, 0.0),
            Point2::new(57.909e9, 0.0), // initial prevposition
        );
        let venus = CelestialObject::new(
            "Venus".to_string(),
            4.869e24,
            Point2::new(108.208e9, 0.0),
            Vector2::new(0.0, 35.02e3),
            Vector2::new(0.0, 0.0),
            Point2::new(108.208e9, 0.0), // initial prevposition
        );
        let earth = CelestialObject::new(
            "Earth".to_string(),
            5.972e24,
            Point2::new(149.596e9, 0.0),
            Vector2::new(0.0, 29.78e3),
            Vector2::new(0.0, 0.0),
            Point2::new(149.596e9, 0.0), // initial prevposition
        );
        let mars = CelestialObject::new(
            "Mars".to_string(),
            6.419e23,
            Point2::new(227.939e9, 0.0),
            Vector2::new(0.0, 24.07e3),
            Vector2::new(0.0, 0.0),
            Point2::new(227.939e9, 0.0), // initial prevposition
        );
        let jupiter = CelestialObject::new(
            "Jupiter".to_string(),
            1.898e27,
            Point2::new(778.299e9, 0.0),
            Vector2::new(0.0, 13.07e3),
            Vector2::new(0.0, 0.0),
            Point2::new(778.299e9, 0.0), // initial prevposition
        );
        let saturn = CelestialObject::new(
            "Saturn".to_string(),
            5.684e26,
            Point2::new(1427.0e9, 0.0),
            Vector2::new(0.0, 9.69e3),
            Vector2::new(0.0, 0.0),
            Point2::new(1427.0e9, 0.0), // initial prevposition
        );
        let uranus = CelestialObject::new(
            "Uranus".to_string(),
            8.681e25,
            Point2::new(2870.972e9, 0.0),
            Vector2::new(0.0, 6.8e3),
            Vector2::new(0.0, 0.0),
            Point2::new(2870.972e9, 0.0), // initial prevposition
        );
        let neptune = CelestialObject::new(
            "Neptune".to_string(),
            1.024e26,
            Point2::new(4497.072e9, 0.0),
            Vector2::new(0.0, 5.43e3),
            Vector2::new(0.0, 0.0),
            Point2::new(4497.072e9, 0.0), // initial prevposition
        );

        StarSystem {
            bodies: vec![sun, mercury, venus, earth, mars, jupiter, saturn, uranus, neptune],
            g: 6.67430e-11, 
        }
    }
}

impl Simulatable for StarSystem {
    fn bodies(&self) -> &Vec<CelestialObject> {
        &self.bodies
    }

    fn simulate(&mut self, dt: f64, num_steps: usize) {
        physics::simulate(&mut self.bodies, dt, num_steps, 0.5);
    }
}