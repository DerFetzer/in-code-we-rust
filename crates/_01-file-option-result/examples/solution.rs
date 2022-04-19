use _01_file_option_result::{Book, ParseBookError};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct BookWrapper(Book);

impl FromStr for BookWrapper {
    type Err = ParseBookError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split(';').map(str::trim).collect();

        if parts.iter().any(|s| s.is_empty()) {
            Err(ParseBookError::InvalidFormat)
        } else {
            match &parts[..] {
                [title, ..] if title.starts_with("//") => Err(ParseBookError::Comment),
                [title, author, price, rest @ ..] if rest.len() <= 1 => Ok(BookWrapper(Book {
                    title: title.to_string(),
                    author: author.to_string(),
                    price: price.parse::<f32>()?,
                    description: rest.first().map(|s| s.to_string()),
                })),
                _ => Err(ParseBookError::InvalidFormat),
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    let f = File::open("input.txt")?;
    let reader = BufReader::new(f);

    let books: Vec<_> = reader
        .lines()
        .flatten()
        .flat_map(|s| s.parse::<BookWrapper>())
        .collect();

    assert_eq!(books.len(), 4);

    println!("{books:?}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn book() {
        let line = "Bussysteme in der Fahrzeugtechnik; Werner Zimmermann, Ralf Schmidgall; 35.96";
        assert_eq!(
            line.parse::<BookWrapper>(),
            Ok(BookWrapper(Book {
                title: "Bussysteme in der Fahrzeugtechnik".to_string(),
                author: "Werner Zimmermann, Ralf Schmidgall".to_string(),
                price: 35.96,
                description: None
            }))
        )
    }

    #[test]
    fn book_with_description() {
        let line = "Bussysteme in der Fahrzeugtechnik; Werner Zimmermann, Ralf Schmidgall; 35.96; Erstes deutschsprachiges Buch zur Informationsverarbeitung im Auto, jetzt mit AUTOSAR 3.x";
        assert_eq!(
            line.parse::<BookWrapper>(),
            Ok(BookWrapper(Book {
                title: "Bussysteme in der Fahrzeugtechnik".to_string(),
                author: "Werner Zimmermann, Ralf Schmidgall".to_string(),
                price: 35.96,
                description: Some("Erstes deutschsprachiges Buch zur Informationsverarbeitung im Auto, jetzt mit AUTOSAR 3.x".to_string())
            }))
        )
    }

    #[test]
    fn comment() {
        let line =
            "// Bussysteme in der Fahrzeugtechnik; Werner Zimmermann, Ralf Schmidgall; 35.96";
        assert_eq!(line.parse::<BookWrapper>(), Err(ParseBookError::Comment))
    }

    #[test]
    fn missing_author() {
        let line = "Bussysteme in der Fahrzeugtechnik; ; 35.96";
        assert_eq!(
            line.parse::<BookWrapper>(),
            Err(ParseBookError::InvalidFormat)
        )
    }

    #[test]
    fn too_short() {
        let line = "Bussysteme in der Fahrzeugtechnik; Werner Zimmermann, Ralf Schmidgall";
        assert_eq!(
            line.parse::<BookWrapper>(),
            Err(ParseBookError::InvalidFormat)
        )
    }

    #[test]
    fn too_long() {
        let line = "Bussysteme in der Fahrzeugtechnik; Werner Zimmermann, Ralf Schmidgall; 35.96; Erstes deutschsprachiges Buch zur Informationsverarbeitung im Auto, jetzt mit AUTOSAR 3.x; Springer";
        assert_eq!(
            line.parse::<BookWrapper>(),
            Err(ParseBookError::InvalidFormat)
        )
    }
}
