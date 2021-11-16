/// is_palindrome is the function which is used to check whether given list is a palindrome or not
///
/// #Arguments
///
/// list - A list is Vector object which contains i32 type numbers
///
/// #Return
///
/// Return Result<bool,String> enum which contains bool for check given list is a palindrome or not and String for handle error
pub fn is_palindrome(list: Vec<i32>) -> Result<bool, String> {
    if list.is_empty() {
        return Err("Given list is not valid".to_string());
    }
    let mut flag = 0;
    let length = list.len();
    let mut index = 0;
    while index <= length / 2 && length != 0 {
        if list[index] != list[length - index - 1] {
            flag = 1;
            break;
        }
        index += 1;
    }
    if flag == 1 {
        Ok(false)
    } else {
        Ok(true)
    }
}
