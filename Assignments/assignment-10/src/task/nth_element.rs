use log::*;

use crate::list::List;
use crate::list::List::{Cons, Nil};

/// nth_index_element function is used to find nth index element in list
///
/// #Argument
///
/// index - index value of i32 type integer
/// count - counter of i32 type integer
/// list - A List of enum type containing Box<i32> and Nil.
///
/// #Return
///
/// Return Result<i32,String>, nth index element in list and handle error as well.
pub fn nth_index_element(index: i32, count: i32, list: List) -> Result<i32, String> {
    match list {
        Cons(current_number, list) => {
            if index == count {
                Ok(current_number)
            } else {
                nth_index_element(index, count + 1, *list)
            }
        }
        Nil => {
            error!("Empty box provided in the given list");
            return Err("Please Provide valid list".to_string());
        }
    }
}
