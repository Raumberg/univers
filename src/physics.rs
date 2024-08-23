// physics.rs

const G: f64 = 6.67430e-11; // gravitational constant in m^3 kg^-1 s^-2

fn calculate_gravitational_force(body1: &CelestialObject, body2: &CelestialObject) -> (f64, f64) {
    // Calculate the distance squared between the two bodies
    let distance_squared = (body2.position.0 - body1.position.0).powi(2) + (body2.position.1 - body1.position.1).powi(2);

    // Calculate the force magnitude
    let force_magnitude = G * body1.mass * body2.mass / distance_squared;

    // Calculate the force components
    let force_x = force_magnitude * (body2.position.0 - body1.position.0) / distance_squared.sqrt();
    let force_y = force_magnitude * (body2.position.1 - body1.position.1) / distance_squared.sqrt();

    (force_x, force_y)
}

fn update_body(body: &mut CelestialObject, bodies: &[CelestialObject], dt: f64) {
    // Initialize the force components to 0
    let mut force_x = 0.0;
    let mut force_y = 0.0;

    // Calculate the total force on the body
    for other_body in bodies {
        if body != other_body {
            let (fx, fy) = calculate_gravitational_force(body, other_body);
            force_x += fx;
            force_y += fy;
        }
    }

    // Calculate the acceleration
    let acceleration_x = force_x / body.mass;
    let acceleration_y = force_y / body.mass;

    // Update the velocity and position
    body.velocity.0 += acceleration_x * dt;
    body.velocity.1 += acceleration_y * dt;
    body.position.0 += body.velocity.0 * dt;
    body.position.1 += body.velocity.1 * dt;
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