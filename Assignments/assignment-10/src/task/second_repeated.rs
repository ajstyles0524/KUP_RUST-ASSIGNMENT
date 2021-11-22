use log::*;

use crate::list::List;
use crate::list::List::{Cons, Nil};

/// second_consecutive_repeated function is used to find second repeating element in list
///
/// #Argument
///
/// temporary_number - An i32 type variable containing the previous value in Cons tuple of List enum.Take default -1
/// count - counter of i32 type integer
/// list - A List of enum type containing Box<i32> and Nil.
///
/// #Return
///
/// Return Result<i32,String>,second repeating element from a given list and handle error as well.
pub fn _second_consecutive_repeated(
    temporary_number: i32,
    count: i32,
    list: List,
) -> Result<i32, String> {
    match list {
        Cons(current_number, list) => {
            if temporary_number == current_number && count == 0 {
                second_consecutive_repeated(current_number, count + 1, *list)
            } else if temporary_number == current_number && count == 1 {
                Ok(current_number)
            } else {
                second_consecutive_repeated(current_number, count, *list)
            }
        }
        Nil => {
            error!("Empty box provided in the given list");
            Err("Please Provide valid list".to_string())
        }
    }
}
