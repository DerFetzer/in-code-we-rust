use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;
use std::sync::{Arc, Mutex};

const BIBLE_URL: &str = "https://www.sermon-online.com/download/german/MartinLuther-1912/Martin_Luther_Uebersetzung_1912.txt";
const CHUNK_SIZE: usize = 1000;

fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    let reader = BufReader::new(File::open(bible)?);
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    let words: Arc<Mutex<HashMap<String, u32>>> = Arc::new(Mutex::new(HashMap::new()));

    let mut handles = vec![];

    for chunk in lines.chunks(CHUNK_SIZE) {
        let owned_chunk = chunk.to_vec();
        let words = words.clone();
        handles.push(std::thread::spawn(move || {
            let mut local_words: HashMap<String, u32> = HashMap::new();
            for line in owned_chunk {
                for word in line.split(' ') {
                    *local_words.entry(word.to_string()).or_insert(1) += 1;
                }
            }
            let mut words = words.lock().unwrap();
            for (word, count) in local_words {
                *words.entry(word).or_insert(count) += count;
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let words = words.lock().unwrap();

    let mut word_list = words.iter().collect::<Vec<_>>();
    word_list.sort_by_key(|e| e.1);
    word_list.reverse();

    println!("{:?}", &word_list[..100]);

    Ok(())
}
