use crate::space::objects::{CelestialObject, CelestialType};
use nalgebra::{Point2, Vector2};
use rayon::prelude::*;
use crate::engine::barnes_hut::{QuadTree, AABB};

pub fn simulate(bodies: &mut Vec<CelestialObject>, dt: f64, num_steps: usize, g: f64) {
    for _step in 0..num_steps {
        let body_count = bodies.len();
        let positions: Vec<(String, Point2<f64>, f64)> = bodies
            .iter()
            .map(|b| (b.name.clone(), b.position, b.mass))
            .collect();
        // Параллельный расчёт сил
        let forces: Vec<Vector2<f64>> = (0..body_count).into_par_iter().map(|i| {
            let mut force = Vector2::new(0.0, 0.0);
            for j in 0..body_count {
                if i != j {
                    let (_, pos_i, mass_i) = &positions[i];
                    let (_, pos_j, mass_j) = &positions[j];
                    let direction = pos_j - pos_i;
                    let distance_squared = direction.norm_squared();
                    if distance_squared > 0.0 {
                        let magnitude = (g * mass_i * mass_j) / distance_squared;
                        force += magnitude * direction.normalize();
                    }
                }
            }
            force
        }).collect();
        // Обновление тел
        bodies.par_iter_mut().enumerate().for_each(|(i, body)| {
            body.acceleration = forces[i] / body.mass;
            body.velocity += body.acceleration * dt;
            body.position += body.velocity * dt;
            body.prevposition = body.position - body.velocity * dt;
        });
        handle_collisions_and_particles(bodies);
    }
}

pub fn simulate_barnes_hut(bodies: &mut Vec<CelestialObject>, dt: f64, num_steps: usize, theta: f64, g: f64) {
    for _step in 0..num_steps {
        // 1. Определяем границы всей системы
        let mut min_x = f64::INFINITY;
        let mut max_x = f64::NEG_INFINITY;
        let mut min_y = f64::INFINITY;
        let mut max_y = f64::NEG_INFINITY;
        for b in bodies.iter() {
            min_x = min_x.min(b.position.x);
            max_x = max_x.max(b.position.x);
            min_y = min_y.min(b.position.y);
            max_y = max_y.max(b.position.y);
        }
        let cx = (min_x + max_x) / 2.0;
        let cy = (min_y + max_y) / 2.0;
        let half_size = ((max_x - min_x).max(max_y - min_y)) / 2.0 + 1.0;
        let boundary = AABB { center: Point2::new(cx, cy), half_size };
        // 2. Строим дерево
        let mut tree = QuadTree::new(boundary);
        for b in bodies.iter() {
            tree = tree.insert(b.clone());
        }
        // 3. Считаем силы для каждого тела
        let forces: Vec<Vector2<f64>> = bodies.par_iter().map(|body| {
            tree.calc_force(body, theta, g)
        }).collect();
        // 4. Обновляем тела
        bodies.par_iter_mut().enumerate().for_each(|(i, body)| {
            body.acceleration = forces[i] / body.mass;
            body.velocity += body.acceleration * dt;
            body.position += body.velocity * dt;
            body.prevposition = body.position - body.velocity * dt;
        });
        handle_collisions_and_particles(bodies);
    }
}

fn handle_collisions_and_particles(bodies: &mut Vec<CelestialObject>) {
    let mut to_remove = vec![];
    let mut to_add = vec![];
    let n = bodies.len();
    let mut collided = vec![false; n];
    // 1. Столкновения (только не-частицы)
    for i in 0..n {
        if bodies[i].kind == CelestialType::Particle { continue; }
        for j in (i+1)..n {
            if bodies[j].kind == CelestialType::Particle { continue; }
            let dist = (bodies[i].position - bodies[j].position).norm();
            let min_dist = bodies[i].radius() + bodies[j].radius();
            if dist < min_dist {
                collided[i] = true;
                collided[j] = true;
                // Генерируем частицы
                let coords = (bodies[i].position.coords + bodies[j].position.coords) / 2.0;
                let pos = nalgebra::Point2::new(coords.x, coords.y);
                let v0 = bodies[i].velocity;
                let v1 = bodies[j].velocity;
                let m0 = bodies[i].mass;
                let m1 = bodies[j].mass;
                let total_mass = m0 + m1;
                let n_particles = 20;
                let lifetime = 200;
                use rand::Rng;
                let mut rng = rand::thread_rng();
                for _ in 0..n_particles {
                    let angle = rng.gen_range(0.0..std::f64::consts::TAU);
                    let speed = rng.gen_range(0.5..2.0) * ((v0.norm() + v1.norm()) / 2.0 + 1.0);
                    let vx = speed * angle.cos();
                    let vy = speed * angle.sin();
                    let mass = total_mass / n_particles as f64 * rng.gen_range(0.5..1.5);
                    let vel = v0 * 0.5 + v1 * 0.5 + Vector2::new(vx, vy);
                    to_add.push(CelestialObject::new_particle(pos, vel, mass, lifetime));
                }
            }
        }
    }
    // 2. Удаляем столкнувшиеся
    for (i, &c) in collided.iter().enumerate() {
        if c { to_remove.push(i); }
    }
    // 3. Обновляем lifetime частиц
    for obj in bodies.iter_mut() {
        if obj.kind == CelestialType::Particle {
            if let Some(life) = obj.lifetime.as_mut() {
                if *life > 0 {
                    *life -= 1;
                }
            }
        }
    }
    // 4. Удаляем умершие частицы
    let mut to_remove_particles = vec![];
    for (i, obj) in bodies.iter().enumerate() {
        if obj.kind == CelestialType::Particle {
            if let Some(life) = obj.lifetime {
                if life == 0 { to_remove_particles.push(i); }
            }
        }
    }
    // 5. Удаляем всё
    to_remove.extend(to_remove_particles);
    to_remove.sort_unstable();
    to_remove.dedup();
    for &i in to_remove.iter().rev() {
        bodies.remove(i);
    }
    // 6. Добавляем новые частицы
    bodies.extend(to_add);
}