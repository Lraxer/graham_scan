use crate::point::Point;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn open_read_graph(filename: &str) -> Result<Vec<Point>,()> {
    let file = File::open(filename).expect("Failed to read point info");

    let mut counter = 0;

    let mut point_vec: Vec<Point> = BufReader::new(file)
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

    // find the bottom left point and move to the first place
    let mut tmp_serial_num = 1u32;
    for i in point_vec.iter() {
        if (point_vec[(tmp_serial_num - 1) as usize].get_y() > i.get_y())
            || ((point_vec[(tmp_serial_num - 1) as usize].get_y() == i.get_y())
                && point_vec[(tmp_serial_num - 1) as usize].get_x() > i.get_x())
        {
            tmp_serial_num = i.get_serial_num().unwrap();
        }
    }

    point_vec.swap(0, (tmp_serial_num-1) as usize);

    // convert all points to vectors to the bottom left point expect the first one
    let length: u32 = point_vec.len() as u32;
    let mut ind: u32 = 1;

    while ind<length {
        let tmp_serial_num = point_vec[ind as usize].get_serial_num();
        point_vec[ind as usize] = point_vec[ind as usize].clone() - point_vec[0].clone();
        point_vec[ind as usize].change_serial_num(tmp_serial_num).unwrap();

        ind += 1;
    }
    
    Ok(point_vec)
}
