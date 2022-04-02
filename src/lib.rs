pub mod point;
pub mod readgraph;
pub mod graham;
pub mod save_edge;

#[cfg(test)]
mod tests {
    use super::*;
    use point::{cross_product, Point};
    // use readgraph::open_read_graph;

    #[test]
    fn test_point_ops() {
        let tmp1 = Point::new(Some(1), 1.5, 12.5).unwrap();
        let tmp2 = Point::new(Some(1), 1.5, 10.5).unwrap();
        assert_eq!((tmp1.clone()+tmp2.clone()).get_x(), 3.0);
        assert_eq!((tmp1.clone()+tmp2.clone()).get_y(), 23.0);
        
        assert_eq!((tmp1.clone()-tmp2.clone()).get_x(), 0.0);
        assert_eq!((tmp1.clone()-tmp2.clone()).get_y(), 2.0);
    }

    #[test]
    fn test_cross_prodcut() {
        let tmp1 = Point::new(Some(1), 1.5, 12.5).unwrap();
        let tmp2 = Point::new(Some(1), 1.5, 10.5).unwrap();
        
        assert_eq!(cross_product(&tmp1, &tmp2), -3.0);
    }
    // fn read_from_file() {
    //     let filename = "../randGraph.txt";
    //     let point_vec = open_read_graph(filename).unwrap();

    //     // pointvec[0].print_point();
    //     // pointvec[1].print_point();
    //     // assert_eq!(point_vec[0].get_serial_num().unwrap(), 1);
    //     // assert_eq!(point_vec.len(), 100);
    //     // assert_eq!(point_vec[point_vec.len()-1].get_serial_num().unwrap(), 100);
    // }
}
