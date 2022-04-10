use crate::point::{cross_product, Point};

pub fn convert_axis(point_vec: &mut [Point]) {
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

    point_vec.swap(0, (tmp_serial_num - 1) as usize);

    // make the first point be the origin
    // convert all points to vectors to the bottom left point
    let length: u32 = point_vec.len() as u32;
    let mut ind: u32 = 1;

    while ind < length {
        let tmp_serial_num = point_vec[ind as usize].get_serial_num();
        point_vec[ind as usize] = point_vec[ind as usize].clone() - point_vec[0].clone();
        point_vec[ind as usize]
            .change_serial_num(tmp_serial_num)
            .unwrap();

        ind += 1;
    }

    let tmp_serial_num = point_vec[0].get_serial_num();
    point_vec[0] = point_vec[0].clone() - point_vec[0].clone();
    point_vec[0].change_serial_num(tmp_serial_num).unwrap();

    point_vec.sort_by(|a, b| a.compare(b));
}

pub fn scan(point_vec: &[Point]) -> Result<Vec<u32>, ()> {
    let mut stack: Vec<&Point> = Vec::new();
    stack.push(&point_vec[0]);
    stack.push(&point_vec[1]);

    let mut ind = 2u32;
    let mut top = 1u32;
    let length = point_vec.len() as u32;
    while ind < length {
        while top >= 1
            && cross_product(
                &(stack[top as usize].clone() - stack[(top - 1) as usize].clone()),
                &(point_vec[ind as usize].clone() - stack[(top - 1) as usize].clone()),
            ) < 1e-10
        {
            stack.pop();
            top -= 1;
        }
        stack.push(&point_vec[ind as usize]);

        ind += 1;
        top += 1;
    }
    stack.push(stack[0]);

    let edge_vec: Vec<u32> = stack.iter().map(|i| i.get_serial_num().unwrap()).collect();

    Ok(edge_vec)
}
