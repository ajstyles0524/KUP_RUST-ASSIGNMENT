use std::collections::HashMap;
/// sum_conditional is the function which return the sum of ages for which the map of name contains the given string
///
/// #Arguments
///
/// map - A map is Hashmap object which contains name and age as key(String) and value(i32 type) respectively
///
///
/// #Return
///
/// Return Result<i32,String> enum,the sum of ages for which the map of name contains the given string and handle the error as well
pub fn sum_conditional(map: HashMap<String, i32>, string: String) -> Result<i32, String> {
    if map.is_empty() {
        return Err("Given hashmap is empty".to_string());
    }
    let mut total = 0;
    let length = string.len();
    for (key, value) in map.iter() {
        for index in 0..key.len() - length + 1 {
            if key[index..index + length] == string {
                total += value;
            }
        }
    }
    Ok(total)
}
