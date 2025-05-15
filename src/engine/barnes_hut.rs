use crate::space::objects::CelestialObject;
use nalgebra::{Point2, Vector2};

#[derive(Debug, Clone)]
pub struct AABB {
    pub center: Point2<f64>,
    pub half_size: f64,
}

impl AABB {
    pub fn contains(&self, point: &Point2<f64>) -> bool {
        (point.x >= self.center.x - self.half_size)
            && (point.x <= self.center.x + self.half_size)
            && (point.y >= self.center.y - self.half_size)
            && (point.y <= self.center.y + self.half_size)
    }
    pub fn quadrant(&self, idx: usize) -> AABB {
        let hs = self.half_size / 2.0;
        let (dx, dy) = match idx {
            0 => (-hs, -hs), // NW
            1 => (hs, -hs),  // NE
            2 => (-hs, hs),  // SW
            3 => (hs, hs),   // SE
            _ => (0.0, 0.0),
        };
        AABB {
            center: Point2::new(self.center.x + dx, self.center.y + dy),
            half_size: hs,
        }
    }
}

#[derive(Debug, Clone)]
pub enum QuadTree {
    Empty(AABB),
    Leaf(AABB, CelestialObject),
    Node {
        boundary: AABB,
        mass: f64,
        com: Point2<f64>,
        children: [Box<QuadTree>; 4],
    },
}

impl QuadTree {
    pub fn new(boundary: AABB) -> Self {
        QuadTree::Empty(boundary)
    }

    pub fn insert(self, body: CelestialObject) -> Self {
        match self {
            QuadTree::Empty(boundary) => QuadTree::Leaf(boundary, body),
            QuadTree::Leaf(boundary, old_body) => {
                // Разбиваем на 4 квадранта
                let mut children = [
                    Box::new(QuadTree::Empty(boundary.quadrant(0))),
                    Box::new(QuadTree::Empty(boundary.quadrant(1))),
                    Box::new(QuadTree::Empty(boundary.quadrant(2))),
                    Box::new(QuadTree::Empty(boundary.quadrant(3))),
                ];
                // Вставляем старое и новое тело
                let mut node = QuadTree::Node {
                    boundary: boundary.clone(),
                    mass: 0.0,
                    com: Point2::new(0.0, 0.0),
                    children,
                };
                node = node.insert(old_body);
                node.insert(body)
            }
            QuadTree::Node { boundary, mut children, .. } => {
                for i in 0..4 {
                    if children[i].boundary().contains(&body.position) {
                        let child = std::mem::replace(&mut children[i], Box::new(QuadTree::Empty(boundary.quadrant(i))));
                        children[i] = Box::new(child.insert(body));
                        break;
                    }
                }
                // После вставки пересчитываем массу и центр масс
                let mut total_mass = 0.0;
                let mut com_x = 0.0;
                let mut com_y = 0.0;
                for c in &children {
                    if let Some((m, pos)) = c.mass_and_com() {
                        total_mass += m;
                        com_x += pos.x * m;
                        com_y += pos.y * m;
                    }
                }
                let com = if total_mass > 0.0 {
                    Point2::new(com_x / total_mass, com_y / total_mass)
                } else {
                    boundary.center
                };
                QuadTree::Node {
                    boundary,
                    mass: total_mass,
                    com,
                    children,
                }
            }
        }
    }

    pub fn boundary(&self) -> &AABB {
        match self {
            QuadTree::Empty(b) => b,
            QuadTree::Leaf(b, _) => b,
            QuadTree::Node { boundary, .. } => boundary,
        }
    }

    pub fn mass_and_com(&self) -> Option<(f64, Point2<f64>)> {
        match self {
            QuadTree::Empty(_) => None,
            QuadTree::Leaf(_, body) => Some((body.mass, body.position)),
            QuadTree::Node { mass, com, .. } => Some((*mass, *com)),
        }
    }

    // Barnes-Hut force calculation
    pub fn calc_force(&self, target: &CelestialObject, theta: f64, g: f64) -> Vector2<f64> {
        match self {
            QuadTree::Empty(_) => Vector2::new(0.0, 0.0),
            QuadTree::Leaf(_, body) => {
                if body.position == target.position {
                    Vector2::new(0.0, 0.0)
                } else {
                    // Обычная сила
                    let dir = body.position - target.position;
                    let dist2 = dir.norm_squared();
                    if dist2 == 0.0 { return Vector2::new(0.0, 0.0); }
                    let f = g * target.mass * body.mass / dist2;
                    f * dir.normalize()
                }
            }
            QuadTree::Node { boundary, mass, com, children } => {
                let s = boundary.half_size * 2.0;
                let d = (com.x - target.position.x).hypot(com.y - target.position.y);
                if d == 0.0 { return Vector2::new(0.0, 0.0); }
                if (s / d) < theta {
                    // Далеко — считаем как одну массу
                    let dir = *com - target.position;
                    let dist2 = dir.norm_squared();
                    if dist2 == 0.0 { return Vector2::new(0.0, 0.0); }
                    let f = g * target.mass * *mass / dist2;
                    f * dir.normalize()
                } else {
                    // Близко — рекурсивно по детям
                    let mut force = Vector2::new(0.0, 0.0);
                    for c in children {
                        force += c.calc_force(target, theta, g);
                    }
                    force
                }
            }
        }
    }
} 