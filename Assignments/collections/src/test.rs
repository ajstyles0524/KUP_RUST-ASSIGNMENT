#[cfg(test)]
mod tests {
    use crate::task_1::hash_map::sum_conditional;
    use crate::task_2::compress_list::compress;
    use crate::task_2::duplicates::add_duplicates;
    use crate::task_2::even::first_even;
    use crate::task_2::palindrome::is_palindrome;
    use crate::task_2::remove::nth_remove;
    use crate::task_2::reverse::reverse_list;

    #[test]
    fn hash_map_success() {
        use std::collections::HashMap;
        let mut first_map = HashMap::new();
        first_map.insert("anu".to_string(), 5);
        first_map.insert("anadi".to_string(), 10);
        first_map.insert("ant".to_string(), 15);
        first_map.insert("aman".to_string(), 20);
        first_map.insert("sachin".to_string(), 25);
        assert_eq!(sum_conditional(first_map, "an".to_string()), Ok(50));
    }

    #[test]
    fn hash_map() {
        use std::collections::HashMap;
        let mut second = HashMap::new();
        second.insert("ekta".to_string(), 1);
        second.insert("ekrashta".to_string(), 2);
        second.insert("djnejb".to_string(), 3);
        second.insert("lkifbu".to_string(), 4);
        second.insert("hamariekta".to_string(), 5);
        assert_eq!(sum_conditional(second, "ek".to_string()), Ok(8));
    }

    #[test]
    fn hash_map_empty() {
        use std::collections::HashMap;
        let third = HashMap::new();
        assert_eq!(
            sum_conditional(third, "ek".to_string()),
            Err("Given hashmap is empty".to_string())
        );
    }

    #[test]
    fn palindrome_success() {
        let list = vec![1, 2, 1];
        assert_eq!(is_palindrome(list), Ok(true));
    }

    #[test]
    fn palindrome_failure() {
        assert_eq!(is_palindrome(vec![1, 2, 3]), Ok(false));
    }

    #[test]
    fn palindrome_empty() {
        assert_eq!(
            is_palindrome(vec![]),
            Err("Given list is not valid".to_string())
        );
    }

    #[test]
    fn reverse_success_one() {
        assert_eq!(reverse_list(vec![1, 2, 3]), Ok(vec![3, 2, 1]));
    }

    #[test]
    fn reverse_success_two() {
        assert_eq!(reverse_list(vec![1, 5, 2]), Ok(vec![2, 5, 1]));
    }

    #[test]
    fn reverse_empty_list() {
        assert_eq!(reverse_list(vec![]), Err("Given list is empty".to_string()));
    }

    #[test]
    fn check_even_success_one() {
        let list = vec![57, 59, 60];
        assert_eq!(first_even(list), Ok(60));
    }

    #[test]
    fn check_even_success_two() {
        let list = vec![0, 1, 2, 3];
        assert_eq!(first_even(list), Ok(0));
    }

    #[test]
    fn check_even_empty_list() {
        let list = vec![];
        assert_eq!(first_even(list), Err("Given list is empty".to_string()));
    }

    #[test]
    fn duplicate_success_one() {
        let list = vec![0, 0, 1, 1];
        assert_eq!(compress(list), Ok(vec![0, 1]));
    }

    #[test]
    fn duplicate_success_two() {
        let list = vec![0, 0, 1, 1, 2, 2, 3, 4, 4, 5];
        assert_eq!(compress(list), Ok(vec![0, 1, 2, 3, 4, 5]));
    }

    #[test]
    fn duplicate_empty_list() {
        let list = vec![];
        assert_eq!(compress(list), Err("Given list is empty".to_string()));
    }

    #[test]
    fn add_duplicate_success_one() {
        let list = vec![1, 2];
        assert_eq!(add_duplicates(list), Ok(vec![1, 1, 2, 2]));
    }

    #[test]
    fn add_duplicate_success_two() {
        let list = vec![1, 2, 3, 4];
        assert_eq!(add_duplicates(list), Ok(vec![1, 1, 2, 2, 3, 3, 4, 4]));
    }

    #[test]
    fn add_duplicate_empty_list() {
        let list = vec![];
        assert_eq!(add_duplicates(list), Err("Given list is empty".to_string()));
    }

    #[test]
    fn remove_nth_value_success() {
        let list = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let nth_value = 3;
        assert_eq!(
            nth_remove(list, nth_value),
            Ok(vec![1, 2, 4, 5, 6, 7, 8, 9])
        );
    }

    #[test]
    fn remove_nth_value() {
        let list = vec![1, 2, 3, 4, 5, 3, 7, 8, 9];
        let nth_value = 3;
        assert_eq!(nth_remove(list, nth_value), Ok(vec![1, 2, 4, 5, 7, 8, 9]));
    }

    #[test]
    fn remove_empty_list() {
        let list = vec![];
        let nth_value = 10;
        assert_eq!(
            nth_remove(list, nth_value),
            Err("Given list is empty".to_string())
        );
    }
}
