use crate::space::objects::{Acceleration, CelestialObject, Force, Velocity};

use nalgebra::{Point2, Vector2};

pub struct QuadTree {
    pub bounds: Rectangle,
    pub capacity: usize,
    pub bodies: Vec<CelestialObject>,
    pub northwest: Option<Box<QuadTree>>,
    pub northeast: Option<Box<QuadTree>>,
    pub southwest: Option<Box<QuadTree>>,
    pub southeast: Option<Box<QuadTree>>,
}

impl QuadTree {
    pub fn new(bounds: Rectangle, capacity: usize) -> Self {
        QuadTree {
            bounds,
            capacity,
            bodies: Vec::new(),
            northwest: None,
            northeast: None,
            southwest: None,
            southeast: None,
        }
    }

    pub fn insert(&mut self, body: CelestialObject) {
        if self.bodies.len() < self.capacity {
            self.bodies.push(body);
        } else {
            if self.northwest.is_none() {
                self.subdivide();
            }
            let index = self.get_index(body.position);
            match index {
                0 => self.northwest.as_mut().unwrap().insert(body),
                1 => self.northeast.as_mut().unwrap().insert(body),
                2 => self.southwest.as_mut().unwrap().insert(body),
                3 => self.southeast.as_mut().unwrap().insert(body),
                _ => unreachable!(),
            }
        }
    }

    pub fn subdivide(&mut self) {
        let x = self.bounds.x;
        let y = self.bounds.y;
        let w = self.bounds.w / 2.0;
        let h = self.bounds.h / 2.0;

        self.northwest = Some(Box::new(QuadTree::new(Rectangle::new(x, y, w, h), self.capacity)));
        self.northeast = Some(Box::new(QuadTree::new(Rectangle::new(x + w, y, w, h), self.capacity)));
        self.southwest = Some(Box::new(QuadTree::new(Rectangle::new(x, y + h, w, h), self.capacity)));
        self.southeast = Some(Box::new(QuadTree::new(Rectangle::new(x + w, y + h, w, h), self.capacity)));
    }

    pub fn get_index(&self, point: Point2<f64>) -> usize {
        let x = point.x;
        let y = point.y;
        let x_mid = self.bounds.x + self.bounds.w / 2.0;
        let y_mid = self.bounds.y + self.bounds.h / 2.0;

        if x <= x_mid && y <= y_mid {
            0
        } else if x > x_mid && y <= y_mid {
            1
        } else if x <= x_mid && y > y_mid {
            2
        } else {
            3
        }
    }

    pub fn calculate_force(&self, body: &CelestialObject, theta: f64) -> Force {
        let distance = body.position - self.bounds.center();
        let distance_squared = distance.norm_squared();

        if distance_squared == 0.0 {
            return Vector2::new(0.0, 0.0);
        }

        let mass = self.total_mass();
        let force = (crate::space::objects::G * body.mass * mass) / distance_squared;
        force * distance.normalize()
    }

    pub fn total_mass(&self) -> f64 {
        self.bodies.iter().map(|body| body.mass).sum::<f64>()
    }

    pub fn traverse(&self, body: &CelestialObject, theta: f64) -> Force {
        let distance = body.position - self.bounds.center();
        let distance_squared = distance.norm_squared();
    
        if self.bodies.len() == 1 {
            self.calculate_force(body, theta)
        } else if distance_squared > (self.bounds.w * theta).powi(2) {
            self.calculate_force(body, theta)
        } else {
            let mut force = Vector2::new(0.0, 0.0);
            if let Some(northwest) = &self.northwest {
                force += northwest.traverse(body, theta);
            }
            if let Some(northeast) = &self.northeast {
                force += northeast.traverse(body, theta);
            }
            if let Some(southwest) = &self.southwest {
                force += southwest.traverse(body, theta);
            }
            if let Some(southeast) = &self.southeast {
                force += southeast.traverse(body, theta);
            }
            force
        }
    }
}

#[derive(Clone, Copy)]
pub struct Rectangle {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
}

impl Rectangle {
    pub fn new(x: f64, y: f64, w: f64, h: f64) -> Self {
        Rectangle { x, y, w, h }
    }

    pub fn center(&self) -> Point2<f64> {
        Point2::new(self.x + self.w / 2.0, self.y + self.h / 2.0)
    }
}

pub fn calculate_force(body: &CelestialObject, quad_tree: &QuadTree, theta: f64) -> Force {
    quad_tree.traverse(body, theta)
}

pub fn update_body(body: &mut CelestialObject, force: Force, dt: f64) {
    body.acceleration = force / body.mass;
    body.velocity += 0.5 * body.acceleration * dt; // Verlet integration 
    body.position += body.velocity * dt;
    body.prevposition = body.position - body.velocity * dt;
}

pub fn simulate(bodies: &mut Vec<CelestialObject>, dt: f64, num_steps: usize, theta: f64) {
    let mut quad_tree = QuadTree::new(Rectangle::new(-1.0, -1.0, 2.0, 2.0), 4);

    for _ in 0..num_steps {
        quad_tree = QuadTree::new(Rectangle::new(-1.0, -1.0, 2.0, 2.0), 4);
        for body in &*bodies {
            quad_tree.insert(body.clone());
        }

        for body in bodies.iter_mut() {
            let force = calculate_force(body, &quad_tree, theta);
            update_body(body, force, dt);
        }
    }
}