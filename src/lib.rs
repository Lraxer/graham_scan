pub mod graham;
pub mod point;
pub mod readgraph;
pub mod save_edge;

#[cfg(test)]
mod tests {
    use super::*;
    use point::{cross_product, distance, Point};

    #[test]
    fn test_point_ops() {
        let tmp1 = Point::new(Some(1), 1.5, 12.5).unwrap();
        let tmp2 = Point::new(Some(1), 1.5, 10.5).unwrap();
        assert_eq!((tmp1.clone() + tmp2.clone()).get_x(), 3.0);
        assert_eq!((tmp1.clone() + tmp2.clone()).get_y(), 23.0);

        assert_eq!((tmp1.clone() - tmp2.clone()).get_x(), 0.0);
        assert_eq!((tmp1.clone() - tmp2.clone()).get_y(), 2.0);
    }

    #[test]
    fn test_cross_prodcut() {
        let tmp1 = Point::new(Some(1), 1.5, 12.5).unwrap();
        let tmp2 = Point::new(Some(1), 1.5, 10.5).unwrap();

        assert_eq!(cross_product(&tmp1, &tmp2), -3.0);
    }

    #[test]
    fn test_get_change_field() {
        let mut tmp1 = Point::new(Some(15), 3.14, 10.5).unwrap();

        assert_eq!(tmp1.get_x(), 3.14);
        assert_eq!(tmp1.get_y(), 10.5);
        assert_eq!(tmp1.get_serial_num().unwrap(), 15);

        tmp1.change_serial_num(Some(10)).unwrap();
        assert_eq!(tmp1.get_serial_num().unwrap(), 10);
    }

    #[test]
    fn dist() {
        let tmp1 = Point::new(Some(1), 1.5, 12.5).unwrap();
        let tmp2 = Point::new(Some(1), 1.5, 10.5).unwrap();

        assert_eq!(distance(&tmp1, &tmp2), 2.0);
    }
}
