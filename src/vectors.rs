use arrayfire::{Array, Dim4};

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Vec2D<T> {
    data: Array<T>,
}

impl<T: Copy> Vec2D<T> {
    pub fn new(x: T, y: T) -> Vec2D<T> {
        Vec2D {
            data: Array::new(&[x, y], Dim4::new(&[2, 1, 1, 1])),
        }
    }

    pub fn zeros() -> Vec2D<T> {
        Vec2D {
            data: Array::zeros(Dim4::new(&[2, 1, 1, 1])),
        }
    }

    pub fn get_x(&self) -> T {
        self.data[0]
    }

    pub fn get_y(&self) -> T {
        self.data[1]
    }

    pub fn norm(&self) -> f64 {
        (self.data[0].powi(2) + self.data[1].powi(2)).sqrt()
    }
}

impl<T: Copy> std::ops::Add for Vec2D<T> {
    type Output = Vec2D<T>;

    fn add(self, other: Vec2D<T>) -> Vec2D<T> {
        Vec2D {
            data: self.data + &other.data,
        }
    }
}

impl<T: Copy> std::ops::Sub for Vec2D<T> {
    type Output = Vec2D<T>;

    fn sub(self, other: Vec2D<T>) -> Vec2D<T> {
        Vec2D {
            data: self.data - &other.data,
        }
    }
}

impl<T: Copy> std::ops::Mul<T> for Vec2D<T> {
    type Output = Vec2D<T>;

    fn mul(self, rhs: T) -> Vec2D<T> {
        Vec2D {
            data: self.data * rhs,
        }
    }
}

impl<T: Copy> std::ops::Div<T> for Vec2D<T> {
    type Output = Vec2D<T>;

    fn div(self, rhs: T) -> Vec2D<T> {
        Vec2D {
            data: self.data / rhs,
        }
    }
}

impl<T: Copy> Vec2D<T> {
    pub fn scalar_mul(&self, rhs: T) -> Vec2D<T> {
        self * rhs
    }
}

impl<T: Copy> std::ops::Mul<Vec2D<T>> for T {
    type Output = Vec2D<T>;

    fn mul(self, rhs: Vec2D<T>) -> Vec2D<T> {
        rhs.scalar_mul(self)
    }
}