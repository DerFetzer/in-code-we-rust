use std::io::BufRead;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let reader = exercise_04::get_and_open_bible()?;

    let _lines: Vec<String> = reader
        .lines()
        .skip(1)
        .flat_map(|l| exercise_04::extract_text_from_line(&l.unwrap()))
        .collect();

    todo!("Count unique words using multiple threads.")
}
