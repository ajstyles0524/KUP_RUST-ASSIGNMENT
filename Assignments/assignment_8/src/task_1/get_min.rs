/// The generic function get_minimum takes input in generic way and it is used to find minimum value
///
/// #Arguments
///
/// input_1: input_1 is a T type input which contains any primitive value
/// input_2: input_2 is a T type input which contains any primitive value
///
/// #Return
///
/// Return Result<T,String> type where T represents minimum value and String for handle error
pub fn get_minimum<T: PartialOrd>(input_1: T, input_2: T) -> Result<T, String> {
    if input_1 < input_2 {
        Ok(input_1)
    } else if input_1 > input_2 {
        Ok(input_2)
    } else {
        Err("Provided inputs are wrong".to_string())
    }
}
