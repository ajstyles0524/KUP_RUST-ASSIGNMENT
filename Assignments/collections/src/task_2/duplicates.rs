/// add_duplicates is a function which is used to append the duplicate element in the given list
///
/// #Arguments
///
/// list - A list is Vector object which contains i32 type integers
///
/// #Return
///
/// Return Result<Vec<i32>,String> enum, vector after appending duplicate element and String for handle error
pub fn add_duplicates(list: Vec<i32>) -> Result<Vec<i32>, String> {
    if list.is_empty() {
        return Err("Given list is empty".to_string());
    }
    let mut vec: Vec<i32> = Vec::new();
    for element in list {
        vec.push(element);
        vec.push(element);
    }
    Ok(vec)
}
