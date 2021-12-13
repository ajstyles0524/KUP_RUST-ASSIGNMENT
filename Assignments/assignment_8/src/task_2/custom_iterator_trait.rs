/// The GeometricSeries structure which is used to store the data(first_number,current_number and ratio)
///
/// #field
///
/// first_number:- a first_number is a i32 integer type object which is first term in GP Series
///
/// current_number:- a current_number is a i32 integer type object which is current term in GP Series
///
/// ratio:- a ratio is a i32 integer type object which is common ration in GP Series
pub struct GeometricSeries {
    pub first_number: i32,
    pub current_number: i32,
    pub ratio: i32,
}
pub trait Iterator {
    fn take(&mut self, size: i32) -> Vec<i32>;
}
impl Iterator for GeometricSeries {
    /// The take method is used return next ( n = size)  number in GP Series
    ///
    /// #Arguments
    ///
    /// size:- A size is a i32 integer type object used to provide next n next number in GP Series
    ///
    /// #Return
    ///
    /// Returns Result enum which contains next n number in GP series and handle Error as Well
    fn take(&mut self, size: i32) ->Vec<i32>{
        let mut series: Vec<i32> = Vec::new();
        for _index in 0..size {
            series.push(self.current_number);
            self.current_number *= self.ratio;
        }
        series
    }
}
