use log::*;
/// Book is a structure which contains information of book
///
/// #field
///
/// accession_number- A accession_number is Vector of i32 type which is used to store accession number of book
///
/// author_name- A author_name is Vector of string which is used to store author of book.
///
/// book_title- A book_title is Vector of string which is used to store title of book
///
/// flag- A flag is bool data type to represent issue status of book
pub struct Book {
    pub accession_number: Vec<i32>,
    pub author_name: Vec<String>,
    pub book_title: Vec<String>,
    pub flag: bool,
}
/// Implementation of method on Book
impl Book {
    /// book_display is a method which is used to display the information of Book
    ///
    /// #Arguments
    ///
    /// &self - reference of the instance of the Book structure
    ///
    /// #Return
    ///
    /// Return Result<String,String> type
    pub fn book_display(&self) -> Result<String, String> {
        if self.accession_number.is_empty() {
            return Err("Not found".to_string());
        }
        for index in 0..self.accession_number.len() {
            info!(
                "accession_number:{},author_name: {} , book_title: {} , ",
                self.accession_number[index], self.author_name[index], self.book_title[index]
            );
        }
        Ok("Book information displayed".to_string())
    }
    /// add_book is a method which is used to add the another book
    ///
    /// #Arguments
    ///
    /// accession_number - accession number of the book of type String.
    ///
    /// author_name - author name of the book of type String.
    ///
    /// book_title - title of the book of type String.
    ///
    /// flag - flag value representing the issue status of book
    ///
    /// #Return
    ///
    /// Return Result type which signify that add a new book or not.
    pub fn add_book(
        &mut self,
        book_title: String,
        author_name: String,
        accession_number: i32,
        flag: bool,
    ) -> Result<bool, i32> {
        if self.accession_number.contains(&accession_number) {
            error!("Hey already exist");
            return Err(0);
        } else {
            self.author_name.push(author_name);
            self.book_title.push(book_title);
            self.accession_number.push(accession_number);
            self.flag = flag;
        }
        Ok(true)
    }
    /// display_book_by_author is a function which is used to display the book which related to given author
    ///
    /// #Arguments
    ///
    /// given_author_name - A given_author_name is a String Object which stores the author_name.
    ///
    /// &self - reference of the instance of the Book structure
    ///
    /// #Return
    ///
    /// Return Result type which signify that given author book is present or not
    pub fn display_book_by_author(&self, given_author_name: String) -> Result<String, String> {
        if !self.author_name.contains(&given_author_name) {
            return Err("No book of this author is present".to_string());
        }
        for index in 0..self.author_name.len() {
            if given_author_name == self.author_name[index] {
                info!(
                    "accession_number:{},author_name: {},book_title: {},",
                    self.accession_number[index], self.author_name[index], self.book_title[index]
                )
            }
        }
        Ok("The book is present of given author".to_string())
    }
    /// display_by_title is a function which is used to display the book which related to given title
    ///
    /// #Arguments
    ///
    /// given_title :- A given_title is a String Object which is store the title_name of book
    ///
    /// &self- reference of the instance of the Book structure
    ///
    /// #Return
    ///
    /// Return Result type which signify that given title book is present or not
    pub fn display_by_title(&self, given_title: String) -> Result<String, String> {
        if !self.book_title.contains(&given_title) {
            return Err("Book is not present for given title".to_string());
        }
        for index in 0..self.book_title.len() {
            if given_title == self.book_title[index] {
                info!(
                    "accession_number:{},author_name: {},book_title: {},",
                    self.accession_number[index], self.author_name[index], self.book_title[index]
                )
            }
        }
        Ok("Book is present for given title ".to_string())
    }
    /// total_books is a function which gives total number of book are present.
    ///
    /// #Argument
    ///
    /// &self- reference of the instance of the Book structure
    ///
    /// #Return
    ///
    /// Return Result type which gives total number of books
    pub fn total_books(&self) -> Result<usize, i32> {
        if self.accession_number.is_empty() {
            error!("No book present");
            Err(0)
        } else {
            Ok(self.accession_number.len())
        }
    }
    /// issue_book is a function which is used to issue a book
    ///
    /// #Arguments
    ///
    /// given_book_title:- given_book_title is a String Object which store the title_name of book
    ///
    /// &mut self- mutable reference of the instance of the Book structure
    ///
    /// #Return
    ///
    /// Return Result type which signify issue book is present or not
    pub fn issue_book(&mut self, given_book_title: String) -> Result<String, String> {
        if !self.book_title.contains(&given_book_title) {
            Err("This book is not present".to_string())
        } else {
            for index in 0..self.book_title.len() - 1 {
                if given_book_title == self.book_title[index] {
                    self.accession_number.remove(index);
                    self.author_name.remove(index);
                    self.book_title.remove(index);
                    self.flag = true;
                }
            }
            Ok("book issued".to_string())
        }
    }
}
