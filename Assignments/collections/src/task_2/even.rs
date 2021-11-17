/// first_even is the function which is used to find out the first even number in the given list
///
/// #Arguments
///
/// list - A list is Vector object which contains i32 type integers
///
/// #Return
///
/// Return Result<i32>,String> enum, first even number and handle the error as well
pub fn first_even(list: Vec<i32>) -> Result<i32, String> {
    if list.is_empty() {
        return Err("Given list is empty".to_string());
    }
    let mut index = 0;
    loop {
        if list[index] % 2 == 0 {
            return Ok(list[index]);
        }
        index += 1;
    }
}
