use std::collections::HashMap;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct Book {
    pub isbn: String,
    pub title: String,
    pub author: String,
}

#[derive(Default)]
struct Stock {
    overall: u32,
    lent: u32,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum LibraryError {
    InvalidOperation,
}

#[derive(Default)]
pub struct Library {
    stock: HashMap<Book, Stock>,
}

impl Library {
    /// Add book to overall stock
    pub fn fill_stock(&mut self, book: Book, amount: u32) {
        self.stock.entry(book).or_default().overall += amount;
    }

    /// Lend book
    pub fn lend(&mut self, book: &Book) -> Result<(), LibraryError> {
        self.stock
            .get_mut(book)
            .filter(|stock| stock.overall - stock.lent >= 1)
            .map(|stock| stock.lent += 1)
            .ok_or(LibraryError::InvalidOperation)
    }

    /// Take back book
    pub fn take_back(&mut self, book: &Book) -> Result<(), LibraryError> {
        self.stock
            .get_mut(book)
            .filter(|stock| stock.lent >= 1)
            .map(|stock| stock.lent -= 1)
            .ok_or(LibraryError::InvalidOperation)
    }

    /// Get overall amount of a book
    pub fn get_overall_amount(&self, book: &Book) -> Option<u32> {
        self.stock.get(book).map(|stock| stock.overall)
    }

    /// Get lent amount of a book
    pub fn get_lent_amount(&self, book: &Book) -> Option<u32> {
        self.stock.get(book).map(|stock| stock.lent)
    }

    /// Get Iterator over all books in the library
    pub fn get_books(&self) -> impl ExactSizeIterator + '_ {
        self.stock.keys()
    }
}

fn main() {
    let mut library = Library::default();

    let book1 = Book {
        isbn: "978-3-658-02418-5".to_string(),
        title: "Bussysteme in der Fahrzeugtechnik".to_string(),
        author: "Werner Zimmermann, Ralf Schmidgall".to_string(),
    };
    let book2 = Book {
        isbn: "978-3-426-28273-1".to_string(),
        title: "Schreib oder stirb".to_string(),
        author: "Micky Beisenherz, Sebastian Fitzek".to_string(),
    };

    assert_eq!(library.get_books().len(), 0);

    library.fill_stock(book1.clone(), 10);
    library.fill_stock(book2.clone(), 20);

    assert_eq!(library.get_books().len(), 2);

    assert_eq!(library.get_overall_amount(&book1).unwrap(), 10);
    assert_eq!(library.get_lent_amount(&book1).unwrap(), 0);

    library.lend(&book1).unwrap();
    library.lend(&book2).unwrap();
    library.lend(&book2).unwrap();

    assert_eq!(library.get_overall_amount(&book2).unwrap(), 20);
    assert_eq!(library.get_lent_amount(&book2).unwrap(), 2);

    library.take_back(&book2).unwrap();

    assert_eq!(library.get_overall_amount(&book2).unwrap(), 20);
    assert_eq!(library.get_lent_amount(&book2).unwrap(), 1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fill_stock() {
        let book1: Book = Book {
            isbn: "1".to_string(),
            title: "".to_string(),
            author: "".to_string(),
        };

        let mut library = Library::default();

        assert_eq!(library.get_overall_amount(&book1), None);
        assert_eq!(library.get_lent_amount(&book1), None);

        library.fill_stock(book1.clone(), 100);

        assert_eq!(library.get_overall_amount(&book1), Some(100));
        assert_eq!(library.get_lent_amount(&book1), Some(0));
    }

    #[test]
    pub fn lend_take_back() {
        let book1: Book = Book {
            isbn: "1".to_string(),
            title: "".to_string(),
            author: "".to_string(),
        };
        let book2: Book = Book {
            isbn: "2".to_string(),
            title: "".to_string(),
            author: "".to_string(),
        };

        let mut library = Library::default();
        library.fill_stock(book1.clone(), 2);

        // Lend
        assert_eq!(library.lend(&book2), Err(LibraryError::InvalidOperation));

        library.lend(&book1).unwrap();
        assert_eq!(library.get_lent_amount(&book1), Some(1));

        library.lend(&book1).unwrap();
        assert_eq!(library.get_lent_amount(&book1), Some(2));

        assert_eq!(library.lend(&book1), Err(LibraryError::InvalidOperation));

        // Take back
        assert_eq!(
            library.take_back(&book2),
            Err(LibraryError::InvalidOperation)
        );

        library.take_back(&book1).unwrap();
        assert_eq!(library.get_lent_amount(&book1), Some(1));

        library.take_back(&book1).unwrap();
        assert_eq!(library.get_lent_amount(&book1), Some(0));

        assert_eq!(
            library.take_back(&book1),
            Err(LibraryError::InvalidOperation)
        );
        assert_eq!(library.get_lent_amount(&book1), Some(0));
    }
}
