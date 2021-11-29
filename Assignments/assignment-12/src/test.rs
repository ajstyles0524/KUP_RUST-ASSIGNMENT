use log::*;
#[cfg(test)]
mod tests {
    use crate::book_library::Book;
    #[test]
    fn display_book_success() {
        env_logger::init();
        let book = Book {
            accession_number: vec![01, 02],
            author_name: vec!["Dave Eggers".to_string(), "Norm Macdonald".to_string()],
            book_title: vec![
                "A Heartbreaking Work of Staggering Genius".to_string(),
                "Based on a True Story".to_string(),
            ],
            flag: false,
        };
        assert_eq!(
            book.book_display(),
            Ok("Book information displayed".to_string())
        );
    }

    #[test]
    fn book_display_failure() {
        let book = Book {
            accession_number: vec![],
            author_name: vec![],
            book_title: vec![],
            flag: false,
        };
        assert_eq!(book.book_display(), Err("Not found".to_string()))
    }

    #[test]
    fn add_book_success() {
        let mut book = Book {
            accession_number: vec![01, 02],
            author_name: vec!["Dave Eggers".to_string(), "Norm Macdonald".to_string()],
            book_title: vec![
                "A Heartbreaking Work of Staggering Genius".to_string(),
                "Based on a True Story".to_string(),
            ],
            flag: false,
        };
        assert_eq!(
            book.add_book(
                "Born Standing Up".to_string(),
                "Steve Martin".to_string(),
                03,
                false
            ),
            Ok(true)
        )
    }

    #[test]
    fn display_book_by_author_success() {
        let book = Book {
            accession_number: vec![01, 02],
            author_name: vec!["Dave Eggers".to_string(), "Norm Macdonald".to_string()],
            book_title: vec![
                "A Heartbreaking Work of Staggering Genius".to_string(),
                "Based on a True Story".to_string(),
            ],
            flag: false,
        };
        assert_eq!(
            book.display_book_by_author("Norm Macdonald".to_string()),
            Ok("The book is present of given author".to_string())
        )
    }

    #[test]
    fn display_book_by_author_failure() {
        let book = Book {
            accession_number: vec![01, 02],
            author_name: vec!["Dave Eggers".to_string(), "Norm Macdonald".to_string()],
            book_title: vec![
                "A Heartbreaking Work of Staggering Genius".to_string(),
                "Based on a True Story".to_string(),
            ],
            flag: false,
        };
        assert_eq!(
            book.display_book_by_author("John Walten".to_string()),
            Err("No book of this author is present".to_string())
        );
    }

    #[test]
    fn display_by_title_success() {
        let book = Book {
            accession_number: vec![01, 02],
            author_name: vec!["Dave Eggers".to_string(), "Norm Macdonald".to_string()],
            book_title: vec![
                "A Heartbreaking Work of Staggering Genius".to_string(),
                "Based on a True Story".to_string(),
            ],
            flag: false,
        };
        assert_eq!(
            book.display_by_title("Based on a True Story".to_string()),
            Ok("Book is present for given title ".to_string())
        )
    }

    #[test]
    fn display_by_title_failure() {
        let book = Book {
            accession_number: vec![01, 02],
            author_name: vec!["Dave Eggers".to_string(), "Norm Macdonald".to_string()],
            book_title: vec![
                "A Heartbreaking Work of Staggering Genius".to_string(),
                "Based on a True Story".to_string(),
            ],
            flag: false,
        };
        assert_eq!(
            book.display_by_title("True lovers".to_string()),
            Err("Book is not present for given title".to_string())
        )
    }

    #[test]
    fn total_book_success() {
        let book = Book {
            accession_number: vec![01, 02],
            author_name: vec!["Dave Eggers".to_string(), "Norm Macdonald".to_string()],
            book_title: vec![
                "A Heartbreaking Work of Staggering Genius".to_string(),
                "Based on a True Story".to_string(),
            ],
            flag: false,
        };
        assert_eq!(book.total_books(), Ok(2))
    }

    #[test]
    fn total_book_failure() {
        let book = Book {
            accession_number: vec![],
            author_name: vec![],
            book_title: vec![],
            flag: false,
        };
        assert_eq!(book.total_books(), Err(0))
    }

    #[test]
    fn issue_book_success() {
        let mut book = Book {
            accession_number: vec![01, 02],
            author_name: vec!["Dave Eggers".to_string(), "Norm Macdonald".to_string()],
            book_title: vec![
                "A Heartbreaking Work of Staggering Genius".to_string(),
                "Based on a True Story".to_string(),
            ],
            flag: false,
        };
        assert_eq!(
            book.issue_book("Based on a True Story".to_string()),
            Ok("book issued".to_string())
        )
    }

    #[test]
    fn issue_book_failure() {
        let mut book = Book {
            accession_number: vec![01, 02],
            author_name: vec!["Dave Eggers".to_string(), "Norm Macdonald".to_string()],
            book_title: vec![
                "A Heartbreaking Work of Staggering Genius".to_string(),
                "Based on a True Story".to_string(),
            ],
            flag: false,
        };
        assert_eq!(
            book.issue_book("True lovers".to_string()),
            Err("This book is not present".to_string())
        )
    }
}
