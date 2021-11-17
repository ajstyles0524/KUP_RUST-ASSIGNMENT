/// nth_remove is the function which is used drop the nth element in the given list
///
/// #Arguments
///
/// list - A list is Vector object which contains i32 type integers
///
/// #Return
///
/// Return Result<Vec<i32>,String>, vector after dropping nth_element and handle the error as well
pub fn nth_remove(list: Vec<i32>, nth_value: i32) -> Result<Vec<i32>, String> {
    if list.is_empty() {
        return Err("Given list is empty".to_string());
    }
    let mut vec = Vec::new();
    for element in list {
        if nth_value != element {
            vec.push(element);
        }
        continue;
    }
    Ok(vec)
}
