use crate::point::{cross_product, Point};

pub fn scan(mut point_vec: Vec<Point>) -> Result<Vec<u32>, ()> {
    // make the first point be the origin
    // let bl_point = point_vec[0].clone();
    let tmp_sn = point_vec[0].get_serial_num();
    point_vec[0] = point_vec[0].clone() - point_vec[0].clone();
    point_vec[0].change_serial_num(tmp_sn).unwrap();

    point_vec.sort_by(|a, b| a.compare(b));

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
