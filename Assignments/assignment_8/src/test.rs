#[cfg(test)]
mod tests {
    use crate::task_1::get_min::get_minimum;
    use crate::task_1::sort_element::sorting_of_elements;
    use crate::task_2::custom_iterator_trait::{GeometricSeries, Iterator};
    #[test]
    fn check_sort_int() {
        let mut arr: [i32; 3] = [5, 10, 0];
        assert_eq!(sorting_of_elements(&mut arr), [0, 5, 10]);
    }
    #[test]
    fn check_sort_float() {
        let mut arr: [f32; 5] = [9.2, 2.2, 7.2, 1.2, 41.2];
        assert_eq!(sorting_of_elements(&mut arr), [1.2, 2.2, 7.2, 9.2, 41.2]);
    }
    #[test]
    fn check_sort_char() {
        let mut arr: [char; 5] = ['e', 'd', 'c', 'b', 'a'];
        assert_eq!(sorting_of_elements(&mut arr), ['a', 'b', 'c', 'd', 'e']);
    }
    #[test]
    fn min_value_for_int_success() {
        assert_eq!(get_minimum(3, 4), Ok(3));
    }
    #[test]
    fn min_value_for_int_failure() {
        assert_eq!(
            get_minimum(4, 4),
            Err("Provided inputs are wrong".to_string())
        );
    }
    #[test]
    fn min_value_for_equal_success() {
        assert_eq!(
            get_minimum(1, 1),
            Err("Provided inputs are wrong".to_string())
        );
    }
    #[test]
    fn min_value_for_equal_failure() {
        assert_eq!(get_minimum(1, 2), Ok(1));
    }
    #[test]
    fn min_value_for_char_success() {
        assert_eq!(get_minimum('b', 'a'), Ok('a'));
    }
    #[test]
    fn min_value_for_char_failure() {
        assert_eq!(
            get_minimum('z', 'z'),
            Err("Provided inputs are wrong".to_string())
        );
    }
    #[test]
    fn min_value_for_float_success() {
        assert_eq!(get_minimum(0.0, 1.1), Ok(0.0));
    }
    #[test]
    fn min_value_for_float_failure() {
        assert_eq!(
            get_minimum(0.5, 0.5),
            Err("Provided inputs are wrong".to_string())
        );
    }
    #[test]
    fn check_iterator_one() {
        let mut gp = GeometricSeries {
            first_number: 1,
            current_number: 1,
            ratio: 2,
        };
        assert_eq!(
            gp.take(11),
            vec![1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024]
        );
    }
    #[test]
    fn check_iterator_two() {
        let mut gp = GeometricSeries {
            first_number: 2,
            current_number: 2,
            ratio: 4,
        };
        assert_eq!(
            gp.take(5),
            vec![2,8,32,128,512]
        );
    }
    #[test]
    fn check_iterator_three() {
        let mut gp = GeometricSeries {
            first_number: 0,
            current_number: 0,
            ratio: 0,
        };
        assert_eq!(gp.take(1), [0]);
    }
}
