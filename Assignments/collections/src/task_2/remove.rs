/// nth_remove is the function which is used drop the nth element in the given list
///
/// #Arguments
///
/// list - A list is Vector object which contains i32 type integers
///
/// #Return
///
/// Return Result<Vec<i32>,String>, vector after dropping nth_element and handle the error as well
pub fn nth_remove(mut list: Vec<i32>, nth_value: i32) -> Result<Vec<i32>, String> {
    if list.is_empty() {
        return Err("Given list is empty".to_string());
    }
    let length = list.len();
    let mut vec: Vec<i32> = Vec::new();
    let mut count = 0;
    let repeat: usize = counting_element(&list, nth_value) as usize;
    for index in 0..length - repeat {
        if list[index + count] == nth_value {
            count += 1;
        }
        list[index] = list[index + count];
    }
    for item in list.iter().take(length - count) {
        vec.push(*item);
    }
    Ok(vec)
}
/// counting_element is the function which is used to count the given element in the given list
///
/// #Arguments
///
/// list - A list is Vector object which contains i32 type integers
/// nth_value - A i32 type integer
///
/// #Return
///
/// Return i32 type integer
pub fn counting_element(list: &[i32], nth_value: i32) -> i32 {
    let mut count = 0;
    for element in list {
        if *element == nth_value {
            count += 1;
        }
    }
    count
}
