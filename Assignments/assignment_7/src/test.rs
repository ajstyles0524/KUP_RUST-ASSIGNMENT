#[cfg(test)]
mod tests {
    use crate::error_handling::check_even_number::response_handle;
    #[test]
    fn check_even_test_1() {
        assert_eq!(response_handle(98), "Even Number".to_string());
    }
    #[test]
    fn check_even_test_2() {
        assert_eq!(response_handle(0), "Even Number".to_string());
    }
    #[test]
    fn error_check_test_1() {
        assert_ne!(
            response_handle(4),
            "Please provide correct input".to_string()
        );
    }
    #[test]
    fn error_check_test_2() {
        assert_ne!(
            response_handle(6),
            "Please provide correct input".to_string()
        );
    }
}
