use log::*;
use std::thread;
use std::time::Duration;
/// The async_function print the data in asynchronous manner.
///
/// #Arguments
///
/// No Arguments.
///
/// #Return
///
/// No return
pub fn async_fashion() {
    let first_value = thread::spawn(|| {
        for index in 6..10 {
            debug!("{}", index);
            thread::sleep(Duration::from_millis(10));
        }
    });
    for index in 1..5 {
        debug!("{}", index);
        thread::sleep(Duration::from_millis(10));
    }
    first_value.join().unwrap()
}
