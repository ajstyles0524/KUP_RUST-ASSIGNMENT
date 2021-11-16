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
    let mut vec: Vec<i32> = Vec::new();
    for element in list {
        let last_element = vec.last();
        match last_element {
            Some(data) => {
                if element != *data {
                    vec.push(element);
                }
            }
            None => vec.push(element),
        }
    }
    Ok(vec)
}
