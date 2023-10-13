use exercise_01::{Book, ParseBookError};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct BookWrapper(Book);

impl FromStr for BookWrapper {
    type Err = ParseBookError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split(';').map(|s| s.trim()).collect();
        if parts[0].starts_with("//") {
            return Err(ParseBookError::Comment);
        }
        for part in &parts {
            if part.is_empty() {
                return Err(ParseBookError::InvalidFormat);
            }
        }
        if parts.len() > 4 || parts.len() <= 2 {
            return Err(ParseBookError::InvalidFormat);
        }
        Ok(BookWrapper(Book {
            title: parts[0].to_string(),
            author: parts[1].to_string(),
            price: parts[2].trim().parse()?,
            description: parts.get(3).map(|s| s.to_string()),
        }))
    }
}

fn main() -> std::io::Result<()> {
    let f = File::open("input.txt")?;
    let reader = BufReader::new(f);

    let books: Vec<BookWrapper> = reader.lines().flatten().flat_map(|z| z.parse()).collect();

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
