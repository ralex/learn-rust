use std::ops::{Add, Deref};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Point {
    x: i32,
    y: i32
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn magnitude(self) -> f64 {
        self.dist(Point::new(0, 0))
    }

    fn dist(self, other_point: Point) -> f64 {
        let a = (self.x as f64 - other_point.x as f64).powi(2);
        let b = (self.y as f64 - other_point.y as f64).powi(2);
        (a + b).sqrt()
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Debug)]
pub struct Polygon {
    points: Vec<Point>
}

impl Polygon {
    fn new() -> Self {
        Self { points: vec![] }
    }

    fn add_point(&mut self, point: Point) {
        self.points.push(point)
    }

    fn left_most_point(self) -> Option<Point> {
        self.points.iter().min_by_key(|p| p.x).cloned()
    }

    fn perimeter(&self) -> f64 {
        let mut perimeter = 0.0;
        let mut last = self.points[0];
        for point in &self.points[1..] {
            perimeter += last.dist(*point);
            last = *point;
        }
        perimeter += last.dist(self.points[0]);
        perimeter
    }
}

impl Deref for Polygon {
    type Target = Vec<Point>;

    fn deref(&self) -> &Self::Target {
        &self.points
    }
}

pub struct Circle {
    center: Point,
    radius: i32,
}

impl Circle {
    fn new(center: Point, radius: i32) -> Self {
        Self { center, radius }
    }

    fn circumference(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius as f64
    }
}

pub enum Shape {
    Polygon(Polygon),
    Circle(Circle),
}

impl From<Polygon> for Shape {
    fn from(polygon: Polygon) -> Self {
        Shape::Polygon(polygon)
    }
}

impl From<Circle> for Shape {
    fn from(circle: Circle) -> Self {
        Shape::Circle(circle)
    }
}

impl Shape {
    fn perimeter(&self) -> f64 {
        match self {
            Self::Polygon(p) => p.perimeter(),
            Self::Circle(c) => c.circumference(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn round_two_digits(x: f64) -> f64 {
        (x * 100.0).round() / 100.0
    }

    #[test]
    fn test_point_magnitude() {
        let p1 = Point::new(12, 13);
        assert_eq!(round_two_digits(p1.magnitude()), 17.69);
    }

    #[test]
    fn test_point_dist() {
        let p1 = Point::new(10, 10);
        let p2 = Point::new(14, 13);
        assert_eq!(round_two_digits(p1.dist(p2)), 5.00);
    }

    #[test]
    fn test_point_add() {
        let p1 = Point::new(16, 16);
        let p2 = p1 + Point::new(-4, 3);
        assert_eq!(p2, Point::new(12, 19));
    }

    #[test]
    fn test_polygon_left_most_point() {
        let p1 = Point::new(12, 13);
        let p2 = Point::new(16, 16);

        let mut poly = Polygon::new();
        poly.add_point(p1.clone());
        poly.add_point(p2);
        assert_eq!(poly.left_most_point(), Some(p1));
    }

    #[test]
    fn test_polygon_iter() {
        let p1 = Point::new(12, 13);
        let p2 = Point::new(16, 16);

        let mut poly = Polygon::new();
        poly.add_point(p1);
        poly.add_point(p2);

        let points = poly.iter().cloned().collect::<Vec<_>>();
        assert_eq!(points, vec![Point::new(12, 13), Point::new(16, 16)]);
    }

    #[test]
    fn test_shape_perimeters() {
        let mut poly = Polygon::new();
        poly.add_point(Point::new(12, 13));
        poly.add_point(Point::new(17, 11));
        poly.add_point(Point::new(16, 16));
        let shapes = vec![
            Shape::from(poly),
            Shape::from(Circle::new(Point::new(10, 20), 5)),
        ];
        let perimeters = shapes
            .iter()
            .map(Shape::perimeter)
            .map(round_two_digits)
            .collect::<Vec<_>>();
        assert_eq!(perimeters, vec![15.48, 31.42]);
    }
}

#[allow(dead_code)]
fn main() {
    let p1 = Point::new(10, 10);
    let p2 = Point::new(14, 13);
    println!("Point 1: {p1:?}");
    println!("Point 1: {p2:?}");
    println!("p1 + p2 = {:?}",p1.add(p2));

    let mut poly = Polygon::new();
    let p1 = Point::new(10, 10);
    let p2 = Point::new(14, 13);
    poly.add_point(p1);
    poly.add_point(p2);
    println!("Polygon: {poly:?}");
}
