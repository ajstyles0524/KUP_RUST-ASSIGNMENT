/// The generic sorting_of_elements which is used to sort the any type of collection of elements
///
/// #Arguments
///
/// list: - A list is Vector object which contains the list of T type
///
/// #Return
///
/// Returns list of T type in sorted order
pub fn sorting_of_elements<T: std::cmp::PartialOrd + Copy>(list: &mut [T]) -> &[T] {
    for index_1 in 0..list.len() {
        let mut min_index = index_1;
        for index_2 in index_1 + 1..list.len() {
            if list[min_index] > list[index_2] {
                min_index = index_2;
            }
        }
        let key = list[min_index];
        while min_index > index_1 {
            list[min_index] = list[min_index - 1];
            min_index -= 1;
        }
        list[index_1] = key;
    }
    list
}
