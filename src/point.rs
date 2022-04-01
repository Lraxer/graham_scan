use std::cmp::Ordering;
use std::ops::{Add, Sub};

#[derive(Clone)]
pub struct Point {
    // None for temp point
    serial_num: Option<u32>,
    x: f64,
    y: f64,
}

impl Point {
    pub fn new(serial_num: Option<u32>, x: f64, y: f64) -> Result<Point, &'static str> {
        Ok(Point { serial_num, x, y })
    }

    pub fn print_point(&self) {
        let tmp = match self.serial_num {
            Some(n) => n.to_string(),
            None => String::from("None"),
        };

        println!("Point: serial num={}, x={}, y={}", tmp, self.x, self.y);
    }

    pub fn get_serial_num(&self) -> Option<u32> {
        self.serial_num
    }

    pub fn change_serial_num(&mut self, new_num: Option<u32>) -> Result<(), ()> {
        self.serial_num = new_num;
        Ok(())
    }

    pub fn get_x(&self) -> f64 {
        self.x
    }

    pub fn get_y(&self) -> f64 {
        self.y
    }

    pub fn compare(&self, b_to_origin: &Point) -> Ordering {
        let res = cross_product(self, b_to_origin);
        if res > 0.0 {
            Ordering::Less
        } else if self.x.powi(2) + self.y.powi(2) < b_to_origin.x.powi(2) + b_to_origin.y.powi(2) {
            Ordering::Greater
        } else {
            Ordering::Greater
        }
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            serial_num: None,
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point {
            serial_num: None,
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

pub fn cross_product(veca: &Point, vecb: &Point) -> f64 {
    veca.x * vecb.y - veca.y * vecb.x
}

pub fn distance(a: &Point, b: &Point) -> f64 {
    ((a.x - b.x).powi(2) + (a.y - b.y).powi(2)).sqrt()
}
