mod objects;
mod physics;
mod system;

use crate::system::{SolarSystem, Simulatable};
use serde_json;

fn main() {
    let mut solar_system = SolarSystem::new();
    let mut json_data = Vec::new();

    for _ in 0..100 {
        solar_system.simulate(1.0, 100);
        let bodies_data: Vec<_> = solar_system.bodies.iter().map(|body| {
            println!("Planet: {}", body.name);
            serde_json::json!({
                "name": body.name.clone(),
                "position": {
                    "x": body.position.x,
                    "y": body.position.y,
                },
                "velocity": {
                    "x": body.velocity.x,
                    "y": body.velocity.y,
                },
                "acceleration": {
                    "x": body.acceleration.x,
                    "y": body.acceleration.y,
                },
            })
        }).collect();
        json_data.push(serde_json::json!(bodies_data));
    }

    let json_output = serde_json::to_string_pretty(&json_data).unwrap();
    std::fs::write("output.json", json_output).unwrap();
}