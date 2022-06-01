use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;

const BIBLE_URL: &str = "https://www.sermon-online.com/download/german/MartinLuther-1912/Martin_Luther_Uebersetzung_1912.txt";

pub fn get_and_open_bible() -> Result<Box<dyn BufRead>, Box<dyn std::error::Error>> {
    let bible = Path::new("bible.txt");
    if !bible.exists() {
        let mut writer = BufWriter::new(File::create(bible)?);
        writer.write_all(
            reqwest::blocking::get(BIBLE_URL)?
                .text_with_charset("windows-1252")
                .unwrap()
                .as_bytes(),
        )?;
    }

    Ok(Box::new(BufReader::new(File::open(bible)?)))
}

pub fn extract_text_from_line(line: &str) -> Option<String> {
    Some(
        line.split_once('§')?
            .1
            .split_once(' ')?
            .1
            .split(" - ")
            .next()?
            .to_string(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_text_from_line() {
        let line = "1. Mose#1#1§1. Am Anfang schuf Gott Himmel und Erde. - Apostelgeschichte 17,24; Offenbarung 4,11; Hebräer 11,3; Johannes 1,1–3.";

        assert_eq!(
            extract_text_from_line(line),
            Some("Am Anfang schuf Gott Himmel und Erde.".to_string())
        )
    }

    #[test]
    fn test_extract_text_from_line_short() {
        let line = "Jesaja#10#6§6. Ich will ihn senden wider ein Heuchelvolk und ihm Befehl tun wider das Volk meines Zorns, daß er’s beraube und austeile und zertrete es wie Kot auf der Gasse,";
        assert_eq!(
            extract_text_from_line(line),
            Some("Ich will ihn senden wider ein Heuchelvolk und ihm Befehl tun wider das Volk meines Zorns, daß er’s beraube und austeile und zertrete es wie Kot auf der Gasse,".to_string())
        )
    }
}
