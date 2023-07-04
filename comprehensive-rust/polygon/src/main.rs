use std::f64::consts::PI;
use std::ops::Add;
use std::slice::Iter;

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Copy, Clone)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y,
        }
    }

    fn magnitude(&self) -> f64 {
        let squares = self.x*self.x + self.y * self.y;
        let squares = squares as f64;
        squares.sqrt()
    }

    fn dist(&self, other: Point) -> f64 {
        let delta_squared = (self.x - other.x).pow(2) + (self.y - other.y).pow(2);
        let delta_squared = delta_squared as f64;
        delta_squared.sqrt()
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

pub struct Polygon {
    points: Vec<Point>,
}

impl Polygon {
    fn new() -> Self {
        Self {
            points: vec!(),
        }
    }

    fn add_point(&mut self, point: Point) {
        self.points.push(point);
    }

    fn iter(&self) -> Iter<Point> {
        self.points.iter()
    }

    fn left_most_point(&self) -> Option<Point> {
        if self.points.is_empty() {
            return None;
        }

        let mut left_most = &self.points[0];
        for point in &self.points {
            if point.x < left_most.x {
                left_most = point;
            }
        }

        Some(*left_most)
    }

    fn perimeter(&self) -> f64 {
        let mut sum: f64 = 0.0;
        for index in 0..self.points.len() {
            let previous = if index > 0 {
                index -1
            } else {
                self.points.len() - 1
            };

            sum += self.points[index].dist(self.points[previous]);
        }

        sum
    }
}

pub struct Circle {
    point: Point,
    radius: i32,
}

impl Circle {
    fn new(point: Point, radius: i32) -> Self {
        Self {
            point,
            radius,
        }
    }

    fn perimeter(&self) -> f64 {
        let radius = self.radius as f64;
        2.0*PI*radius
    }
}

pub enum Shape {
    Polygon(Polygon),
    Circle(Circle),
}

impl Shape {
    fn perimeter(&self) -> f64 {
        match self {
            Shape::Polygon(polygon) => polygon.perimeter(),
            Shape::Circle(circle) => circle.perimeter(),
        }
    }
}

impl From<Polygon> for Shape {
    fn from(polygon: Polygon) -> Self {
        Shape::Polygon(Polygon {
            points: polygon.points,
        })
    }
}

impl From<Circle> for Shape {
    fn from(circle: Circle) -> Self {
        Shape::Circle(Circle {
           point: circle.point,
           radius: circle.radius,
        })
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

        let mut polygon = Polygon::new();
        polygon.add_point(p1);
        polygon.add_point(p2);
        assert_eq!(polygon.left_most_point(), Some(p1));
    }

    #[test]
    fn test_polygon_iter() {
        let p1 = Point::new(12, 13);
        let p2 = Point::new(16, 16);

        let mut polygon = Polygon::new();
        polygon.add_point(p1);
        polygon.add_point(p2);

        let points = polygon.iter().cloned().collect::<Vec<_>>();
        assert_eq!(points, vec![Point::new(12, 13), Point::new(16, 16)]);
    }

    #[test]
    fn test_circle_perimeter() {
        let circle = Circle::new(Point::new(0, 0), 5);
        assert_eq!(round_two_digits(circle.perimeter()), 31.42);
    }

    #[test]
    fn test_polygon_perimeter() {
        let mut polygon = Polygon::new();
        polygon.add_point(Point::new(12, 13));
        polygon.add_point(Point::new(17, 11));
        polygon.add_point(Point::new(16, 16));
        assert_eq!(round_two_digits(polygon.perimeter()), 15.48);
    }

    #[test]
    fn test_polygon_perimeter_via_shape() {
        let mut polygon = Polygon::new();
        polygon.add_point(Point::new(12, 13));
        polygon.add_point(Point::new(17, 11));
        polygon.add_point(Point::new(16, 16));

        let shape = Shape::from(polygon);
        assert_eq!(round_two_digits(shape.perimeter()), 15.48);
    }

    #[test]
    fn test_circle_perimeter_via_shape() {
        let circle = Circle::new(Point::new(0, 0), 5);
        assert_eq!(round_two_digits(circle.perimeter()), 31.42);

        let shape = Shape::from(circle);
        assert_eq!(round_two_digits(shape.perimeter()), 31.42);
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
fn main() {}