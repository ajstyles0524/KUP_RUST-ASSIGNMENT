/// reverse_list is a function which reverse the list of i32 type integers
///
/// #Arguments
///
/// list - A list is Vector object which contains i32 type integers
///
/// #Return
///
/// Return Result<Vec<i32>,String> enum, the reverse of list and handle error as well
pub fn reverse_list(mut list: Vec<i32>) -> Result<Vec<i32>, String> {
    if list.is_empty() {
        return Err("Given list is empty".to_string());
    }
    let mut start_index = 0;
    let mut end_index = list.len() - 1;
    let mut temp;
    while start_index < end_index {
        temp = list[start_index];
        list[start_index] = list[end_index];
        list[end_index] = temp;
        start_index += 1;
        end_index -= 1;
    }
    Ok(list)
}
