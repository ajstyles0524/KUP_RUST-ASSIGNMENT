#[cfg(test)]
mod tests {
    use crate::error_handling::check_even_number::response_handle;
    #[test]
    fn check_even_success_first(){
        assert_eq!(response_handle(98), "Even Number".to_string());
    }
    #[test]
    fn check_even_success_second(){
        assert_eq!(response_handle(0), "Even Number".to_string());
    }
    #[test]
    fn check_even_failure_first(){
        assert_ne!(
            response_handle(3),
            "Please provide correct input".to_string()
        );
    }
    #[test]
    fn check_even_failure_second(){
        assert_ne!(
            response_handle(7),
            "Please provide correct input".to_string()
        );
    }
}
