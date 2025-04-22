use crate::space::objects::{CelestialObject};
use nalgebra::{Point2, Vector2};

pub fn simulate(bodies: &mut Vec<CelestialObject>, dt: f64, num_steps: usize, _theta: f64) {
    for _step in 0..num_steps {
        // Direct N-body calculation
        let body_count = bodies.len();
        
        // Calculate forces directly - this is O(n²) but more reliable
        let mut forces = Vec::with_capacity(body_count);
        
        // Store current positions to avoid borrowing issues
        let positions: Vec<(String, Point2<f64>, f64)> = bodies
            .iter()
            .map(|b| (b.name.clone(), b.position, b.mass))
            .collect();
            
        // Calculate forces for each body
        for i in 0..body_count {
            let mut force = Vector2::new(0.0, 0.0);
            
            for j in 0..body_count {
                if i != j {  // Don't calculate force with itself
                    let (_, pos_i, mass_i) = &positions[i];
                    let (_, pos_j, mass_j) = &positions[j];
                    
                    // Calculate direction vector (from body i to body j)
                    let direction = pos_j - pos_i;
                    let distance_squared = direction.norm_squared();
                    
                    if distance_squared > 0.0 {
                        // Calculate gravitational force (F = G * m1 * m2 / r²)
                        let magnitude = (crate::space::objects::G * mass_i * mass_j) / distance_squared;
                        
                        // Force is in the direction of the other body (attractive)
                        force += magnitude * direction.normalize();
                    }
                }
            }
            
            forces.push(force);
        }
        
        // Update all bodies with calculated forces
        for (i, body) in bodies.iter_mut().enumerate() {
            body.acceleration = forces[i] / body.mass;
            body.velocity += body.acceleration * dt;
            body.position += body.velocity * dt;
            body.prevposition = body.position - body.velocity * dt;
        }
    }
}