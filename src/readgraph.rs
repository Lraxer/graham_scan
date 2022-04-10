use crate::point::Point;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn open_read_graph(filename: &str) -> Result<Vec<Point>, ()> {
    let file = File::open(filename).expect("Failed to read point info");

    let mut counter = 0;

    let point_vec: Vec<Point> = BufReader::new(file)
        .lines()
        .map(|tmpstr| {
            let tmpvec: Vec<f64> = tmpstr
                .unwrap()
                .split(',')
                .map(|numstr| numstr.parse::<f64>().unwrap())
                .collect();
            counter = counter + 1;
            Point::new(Some(counter), tmpvec[0], tmpvec[1]).unwrap()
        })
        .collect();

    Ok(point_vec)
}
