use std::ops::{Add, Sub};

// Point
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Point {
    x: f32,
    y: f32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Self { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Self { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

#[cfg(test)]
mod tests {
    use crate::structures::*;

    #[test]
    fn point_math() {
        let a = Point::new(10.0, 10.0);
        let b = Point::new(10.0, 10.0);

        assert_eq!(a + b, Point::new(20.0, 20.0));
        assert_eq!(a - b, Point::default());
    }

    #[test]
    fn sizes() {
    }
}
