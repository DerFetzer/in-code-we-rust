use exercise_03::{Book, Library};

// TODO: 1. Implement Library
// TODO: 2. Add transaction log

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

    // assert_eq!(library.get_books().len(), 0); // TODO:

    library.fill_stock(book1.clone(), 10);
    library.fill_stock(book2.clone(), 20);

    // assert_eq!(library.get_books().len(), 2); // TODO:

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
