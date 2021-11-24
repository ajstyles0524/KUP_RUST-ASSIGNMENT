use async_std::task;
use futures::future::join;
use std::time::Duration;
/// print_table which used to async function that polls two futures simultaneously to print two tables in asynchronous manner.
///
/// #Arguments
///
/// No Arguments.
///
/// #Return
///
/// No return
pub async fn print_table() {
    let two_table = async {
        for index in 1..11 {
            let value = 2 * index;
            println!("{}", value);
        }
        task::sleep(Duration::from_secs(1)).await;
    };
    let third_table = async {
        for index in 1..11 {
            let value = 3 * index;
            println!("{}", value);
        }
        task::sleep(Duration::from_secs(1)).await;
    };
    join(two_table, third_table).await;
    log::info!("The table printed asynchronously")
}
