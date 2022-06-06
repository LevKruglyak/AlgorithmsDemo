use std::ops::{Add, Mul, Sub};

/// Simple 2D point type
#[derive(Debug, Default, Clone, Copy, PartialEq)]
struct Point {
    x: f32,
    y: f32,
}

impl Point {
    fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// Returns the square of the distance to another point
    fn sq_distance(&self, other: Point) -> f32 {
        (other.x - self.x) * (other.x - self.x) + (other.y - self.y) * (other.y - self.y)
    }

    /// Returns the distance to another point
    fn distance(&self, other: Point) -> f32 {
        ((other.x - self.x) * (other.x - self.x) + (other.y - self.y) * (other.y - self.y)).sqrt()
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul<f32> for Point {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

/// Rectangle specified by a center point and radius
#[derive(Debug, Default, Clone, Copy, PartialEq)]
struct CenterSquare {
    center: Point,
    radius: f32,
}

impl CenterSquare {
    pub fn new(center: Point, radius: f32) -> Self {
        Self { center, radius }
    }

    /// Returns the northeast quadrant of the square
    pub fn quad_ne(&self) -> Self {
        Self {
            center: self.center + Point::new(self.radius / 2.0, self.radius / 2.0),
            radius: self.radius / 2.0,
        }
    }

    /// Returns the northwest quadrant of the square
    pub fn quad_nw(&self) -> Self {
        Self {
            center: self.center + Point::new(-self.radius / 2.0, self.radius / 2.0),
            radius: self.radius / 2.0,
        }
    }

    /// Returns the southeast quadrant of the square
    pub fn quad_se(&self) -> Self {
        Self {
            center: self.center + Point::new(self.radius / 2.0, -self.radius / 2.0),
            radius: self.radius / 2.0,
        }
    }

    /// Returns the southwest quadrant of the square
    pub fn quad_sw(&self) -> Self {
        Self {
            center: self.center + Point::new(-self.radius / 2.0, -self.radius / 2.0),
            radius: self.radius / 2.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::plane::{Point, CenterSquare};

    #[test]
    fn point() {
        let a = Point::new(1.0, 2.0);
        let b = Point::new(2.0, 3.0);

        assert_eq!(a + b, Point::new(3.0, 5.0));
        assert_eq!(a - b, Point::new(-1.0, -1.0));
        assert_eq!(a * 2.0, Point::new(2.0, 4.0));
    }

    #[test]
    fn distance() {
        let a = Point::new(1.0, 5.0);
        let b = Point::new(4.0, 1.0);

        assert_eq!(a.distance(b), 5.0);
        assert_eq!(a.sq_distance(b), 25.0);
    }

    #[test]
    fn quad() {
        let sq = CenterSquare::new(Point::new(0.0, 0.0), 1.0);

        assert_eq!(sq.quad_ne(), CenterSquare::new(Point::new(0.5, 0.5), 0.5));
        assert_eq!(sq.quad_nw(), CenterSquare::new(Point::new(-0.5, 0.5), 0.5));
        assert_eq!(sq.quad_se(), CenterSquare::new(Point::new(0.5, -0.5), 0.5));
        assert_eq!(sq.quad_sw(), CenterSquare::new(Point::new(-0.5, -0.5), 0.5));
    }
}
