#[cfg(test)]
mod tests {
    use crate::list::List::{Cons, Nil};
    use crate::task::first_repeated::first_consecutive_repeated;
    use crate::task::nth_element::nth_index_element;
    use crate::task::second_repeated::second_consecutive_repeated;
    use crate::task::third_odd::third_odd_search;

    #[test]
    fn first_repeated_success() {
        env_logger::init();
        let list = Cons(
            1,
            Box::new(Cons(
                21,
                Box::new(Cons(
                    21,
                    Box::new(Cons(4, Box::new(Cons(5, Box::new(Cons(5, Box::new(Nil))))))),
                )),
            )),
        );
        assert_eq!(first_consecutive_repeated(-1, list), Ok(21));
    }

    #[test]
    fn first_repeated() {
        let list = Cons(
            1,
            Box::new(Cons(
                1,
                Box::new(Cons(
                    21,
                    Box::new(Cons(
                        21,
                        Box::new(Cons(5, Box::new(Cons(5, Box::new(Nil))))),
                    )),
                )),
            )),
        );
        assert_eq!(first_consecutive_repeated(-1, list), Ok(1));
    }

    #[test]
    fn first_repeated_failure() {
        let list = Cons(1, Box::new(Nil));
        assert_eq!(
            first_consecutive_repeated(-1, list),
            Err("Please Provide valid list".to_string())
        );
    }

    #[test]
    fn second_repeated_success() {
        let list = Cons(
            2,
            Box::new(Cons(
                3,
                Box::new(Cons(
                    5,
                    Box::new(Cons(5, Box::new(Cons(2, Box::new(Cons(2, Box::new(Nil))))))),
                )),
            )),
        );
        assert_eq!(second_consecutive_repeated(-1, 0, list), Ok(2));
    }

    #[test]
    fn second_success() {
        let list = Cons(
            1,
            Box::new(Cons(
                1,
                Box::new(Cons(
                    3,
                    Box::new(Cons(5, Box::new(Cons(5, Box::new(Cons(2, Box::new(Nil))))))),
                )),
            )),
        );
        assert_eq!(second_consecutive_repeated(-1, 0, list), Ok(5));
    }

    #[test]
    fn second_repeated_failure() {
        let list = Cons(
            0,
            Box::new(Cons(
                1,
                Box::new(Cons(
                    2,
                    Box::new(Cons(3, Box::new(Cons(4, Box::new(Cons(5, Box::new(Nil))))))),
                )),
            )),
        );
        assert_eq!(
            second_consecutive_repeated(-1, 0, list),
            Err("Please Provide valid list".to_string())
        );
    }

    #[test]
    fn nth_element_success() {
        let list = Cons(
            1,
            Box::new(Cons(
                2,
                Box::new(Cons(
                    3,
                    Box::new(Cons(4, Box::new(Cons(5, Box::new(Cons(6, Box::new(Nil))))))),
                )),
            )),
        );
        assert_eq!(nth_index_element(3, 0, list), Ok(4));
    }

    #[test]
    fn nth_element() {
        let list = Cons(
            1,
            Box::new(Cons(
                2,
                Box::new(Cons(
                    3,
                    Box::new(Cons(4, Box::new(Cons(5, Box::new(Cons(6, Box::new(Nil))))))),
                )),
            )),
        );
        assert_eq!(nth_index_element(1, 0, list), Ok(2));
    }

    #[test]
    fn nth_element_failure() {
        let list = Cons(
            1,
            Box::new(Cons(
                2,
                Box::new(Cons(
                    3,
                    Box::new(Cons(4, Box::new(Cons(5, Box::new(Cons(6, Box::new(Nil))))))),
                )),
            )),
        );
        assert_eq!(
            nth_index_element(10, 0, list),
            Err("Please Provide valid list".to_string())
        );
    }

    #[test]
    fn third_odd_success() {
        let list = Cons(
            1,
            Box::new(Cons(
                2,
                Box::new(Cons(
                    3,
                    Box::new(Cons(4, Box::new(Cons(5, Box::new(Cons(6, Box::new(Nil))))))),
                )),
            )),
        );
        assert_eq!(third_odd_search(0, list), Ok(5));
    }

    #[test]
    fn third_odd() {
        let list = Cons(
            1,
            Box::new(Cons(
                21,
                Box::new(Cons(3, Box::new(Cons(4, Box::new(Cons(5, Box::new(Nil))))))),
            )),
        );
        assert_eq!(third_odd_search(0, list), Ok(3));
    }

    #[test]
    fn third_odd_failure() {
        let list = Cons(
            2,
            Box::new(Cons(
                4,
                Box::new(Cons(3, Box::new(Cons(6, Box::new(Cons(5, Box::new(Nil))))))),
            )),
        );
        assert_eq!(
            third_odd_search(0, list),
            Err("Please Provide valid list".to_string())
        );
    }
}
