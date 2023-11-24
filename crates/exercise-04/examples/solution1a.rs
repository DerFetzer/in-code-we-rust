use std::collections::HashMap;
use std::io::BufRead;
use std::sync::Mutex;
use std::time::Instant;

const CHUNK_SIZE: usize = 1000;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let reader = exercise_04::get_and_open_bible()?;

    let start = Instant::now();

    let lines: Vec<String> = reader
        .lines()
        .flat_map(|l| exercise_04::extract_text_from_line(&l.unwrap()))
        .collect();

    let words: Mutex<HashMap<String, u32>> = Mutex::new(HashMap::new());

    std::thread::scope(|s| {
        for chunk in lines.chunks(CHUNK_SIZE) {
            let words = &words;

            s.spawn(move || {
                let mut local_words: HashMap<String, u32> = HashMap::new();
                for line in chunk {
                    for word in line.replace([',', '.', ';'], "").to_lowercase().split(' ') {
                        if let Some(num) = local_words.get_mut(word) {
                            *num += 1;
                        } else {
                            local_words.insert(word.to_string(), 1);
                        }
                    }
                }
                let mut words = words.lock().unwrap();
                for (word, count) in local_words {
                    *words.entry(word).or_default() += count;
                }
            });
        }
    });

    let words = words.lock().unwrap();

    let mut word_list = words.iter().collect::<Vec<_>>();
    word_list.sort_by_key(|e| e.1);
    word_list.reverse();

    println!(
        "Duration: {}ms",
        Instant::now().duration_since(start).as_millis()
    );
    println!("{} unique words", word_list.len());
    println!("{:?}", &word_list[..100]);

    Ok(())
}
