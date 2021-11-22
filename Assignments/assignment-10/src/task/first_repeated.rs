use log::*;
use crate::list::List;
use crate::list::List::{Cons, Nil};

/// first_consecutive_repeated function is used to find first repeating element in list
///
/// #Argument
///
/// temporary_number - An i32 type variable containing the previous value in Cons tuple of List enum.Take default -1
/// list - A List of enum type containing Box<i32> and Nil.
///
/// #Return
///
/// Return Result<i32,String>,first repeating element from a given list and handle error as well.
pub fn first_consecutive_repeated(temporary_number: i32, list: List) -> Result<i32, String> {
    match list {
        Cons(current_number, list) => {
            if temporary_number == current_number {
                Ok(current_number)
            } else {
                first_consecutive_repeated(current_number, *list)
            }
        }

        Nil => {
            error!("Empty box provided in the given list");
            Err("Please Provide valid list".to_string())
        }
    }
}
