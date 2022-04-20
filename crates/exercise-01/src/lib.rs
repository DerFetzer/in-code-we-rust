use std::num::ParseFloatError;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum ParseBookError {
    Comment,
    InvalidFormat,
}

impl From<ParseFloatError> for ParseBookError {
    fn from(_: ParseFloatError) -> Self {
        Self::InvalidFormat
    }
}

#[derive(Debug, PartialEq)]
pub struct Book {
    pub title: String,
    pub author: String,
    pub price: f32,
    pub description: Option<String>,
}

impl FromStr for Book {
    type Err = ParseBookError;

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn book() {
        let line = "Bussysteme in der Fahrzeugtechnik; Werner Zimmermann, Ralf Schmidgall; 35.96";
        assert_eq!(
            line.parse::<Book>(),
            Ok(Book {
                title: "Bussysteme in der Fahrzeugtechnik".to_string(),
                author: "Werner Zimmermann, Ralf Schmidgall".to_string(),
                price: 35.96,
                description: None
            })
        )
    }

    #[test]
    fn book_with_description() {
        let line = "Bussysteme in der Fahrzeugtechnik; Werner Zimmermann, Ralf Schmidgall; 35.96; Erstes deutschsprachiges Buch zur Informationsverarbeitung im Auto, jetzt mit AUTOSAR 3.x";
        assert_eq!(
            line.parse::<Book>(),
            Ok(Book {
                title: "Bussysteme in der Fahrzeugtechnik".to_string(),
                author: "Werner Zimmermann, Ralf Schmidgall".to_string(),
                price: 35.96,
                description: Some("Erstes deutschsprachiges Buch zur Informationsverarbeitung im Auto, jetzt mit AUTOSAR 3.x".to_string())
            })
        )
    }

    #[test]
    fn comment() {
        let line =
            "// Bussysteme in der Fahrzeugtechnik; Werner Zimmermann, Ralf Schmidgall; 35.96";
        assert_eq!(line.parse::<Book>(), Err(ParseBookError::Comment))
    }

    #[test]
    fn missing_author() {
        let line = "Bussysteme in der Fahrzeugtechnik; ; 35.96";
        assert_eq!(line.parse::<Book>(), Err(ParseBookError::InvalidFormat))
    }

    #[test]
    fn too_short() {
        let line = "Bussysteme in der Fahrzeugtechnik; Werner Zimmermann, Ralf Schmidgall";
        assert_eq!(line.parse::<Book>(), Err(ParseBookError::InvalidFormat))
    }

    #[test]
    fn too_long() {
        let line = "Bussysteme in der Fahrzeugtechnik; Werner Zimmermann, Ralf Schmidgall; 35.96; Erstes deutschsprachiges Buch zur Informationsverarbeitung im Auto, jetzt mit AUTOSAR 3.x; Springer";
        assert_eq!(line.parse::<Book>(), Err(ParseBookError::InvalidFormat))
    }
}
