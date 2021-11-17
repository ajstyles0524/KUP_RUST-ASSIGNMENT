/// compress is the function which is used to remove continuously occurring duplicate from the given list
///
/// #Arguments
///
/// list - A list is Vector object which contains i32 type integers
///
/// #Return
///
/// Return Result<Vec<i32>,String> which contains vector after removing continuously occurring duplicates and  String for handle error
pub fn compress(list: Vec<i32>) -> Result<Vec<i32>, String> {
    if list.is_empty() {
        return Err("Given list is empty".to_string());
    }
    let mut vec = Vec::new();
    let length = list.len();
    vec.push(list[0]);
    for index_1 in 1..length {
        if list[index_1] != list[index_1 - 1] {
            vec.push(list[index_1]);
        }
    }
    Ok(vec)
}
