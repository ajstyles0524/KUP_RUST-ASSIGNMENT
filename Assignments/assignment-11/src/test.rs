#[cfg(test)]
mod tests {
    use crate::async_data::async_fashion;
    use crate::print_table::print_table;
    use futures::executor::block_on;

    #[test]
    fn async_fashion_generator() {
        assert_eq!(async_fashion(), ())
    }

    #[test]
    fn print_table_success() {
        assert_eq!(block_on(print_table()), ())
    }
}
