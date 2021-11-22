use crate::list::List;
use crate::list::List::{Cons, Nil};
use log::*;

/// odd_check function is used to find whether given number is odd or not
///
/// #Argument
///
/// current_number - a i32 type integer
///
/// #Return
///
/// Return bool type(true,false)
pub fn odd_check(current_number: i32) -> bool {
    current_number % 2 != 0
}
/// third_odd_search function is used to find third odd element in list
///
/// #Argument
///
/// count - counter of i32 type integer
/// list - A List of enum type containing Box<i32> and Nil.
///
/// #Return
///
/// Return Result<i32,String>,third odd element in list and handle error as well.
pub fn third_odd_search(count: i32, list: List) -> Result<i32, String> {
    match list {
        Cons(current_number, list) => {
            let current_value = odd_check(current_number);
            match current_value {
                true => {
                    let status: bool = (count == 0) || (count == 1);
                    match status {
                        true => third_odd_search(count + 1, *list),
                        false => Ok(current_number),
                    }
                }
                false => third_odd_search(count, *list),
            }
        }
        Nil => {
            error!("Empty box provided in the given list");
            Err("Please Provide valid list".to_string())
        }
    }
}
