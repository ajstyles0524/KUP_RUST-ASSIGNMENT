/// List - An enum type data
///
/// Variant
///
/// Cons(i32,Box<List>) - tuple object having i32 integer and Box<List>
/// Nil - Signal the end of the list
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}
