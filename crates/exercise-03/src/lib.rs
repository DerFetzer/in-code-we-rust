use std::collections::HashMap;

#[derive(Debug, Clone)]
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

#[derive(Debug, Copy, Clone)]
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
        todo!()
    }

    /// Lend book
    pub fn lend(&mut self, book: &Book) -> Result<(), LibraryError> {
        todo!()
    }

    /// Take back book
    pub fn take_back(&mut self, book: &Book) -> Result<(), LibraryError> {
        todo!()
    }

    /// Get overall amount of a book
    pub fn get_overall_amount(&self, book: &Book) -> Option<u32> {
        todo!()
    }

    /// Get lent amount of a book
    pub fn get_lent_amount(&self, book: &Book) -> Option<u32> {
        todo!()
    }

    /// Get all books
    pub fn get_books(&self) -> () /*TODO*/ {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fill_stock() {
        todo!()
    }

    #[test]
    pub fn lend_take_back() {
        todo!()
    }
}
