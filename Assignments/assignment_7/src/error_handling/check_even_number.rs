/// The is_even function is used to check a given input number is even or not
///
/// #Arguments
///
/// input_number: A input_number is a integer type input number whose use to check even or not
///
/// #Return
///
/// Return Result enum which is used to handle error and value
fn is_even(input_number: i32) -> Result<String, String> {
    if input_number % 2 == 0 {
        Ok("Even Number".to_string())
    } else {
        Err("Non Even Number".to_string())
    }
}
/// The response_handle function is used handle response of calling function.
///
/// #Arguments
///
/// input_number: The input_number is a integer type input number whose use to check even or not with handle error and value
///
/// #Return
///
/// Return String which handle error and value of output of function.
pub fn response_handle(input_number: i32) -> String {
    let response = is_even(input_number);
    match response {
        Ok(_) => "Even Number".to_string(),
        Err(_) => "Please provide correct input".to_string(),
    }
}
