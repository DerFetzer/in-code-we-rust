use std::collections::HashMap;
use std::io::BufRead;
use std::time::Instant;

const CHUNK_SIZE: usize = 1000;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let reader = exercise_04::get_and_open_bible()?;

    let start = Instant::now();

    let lines: Vec<String> = reader
        .lines()
        .flat_map(|l| exercise_04::extract_text_from_line(&l.unwrap()))
        .collect();

    let mut handles = vec![];

    let (tx, rx) = std::sync::mpsc::channel();

    for chunk in lines.chunks(CHUNK_SIZE) {
        let tx = tx.clone();
        let owned_chunk = chunk.to_vec();

        handles.push(std::thread::spawn(move || {
            let mut local_words: HashMap<String, u32> = HashMap::new();
            for line in owned_chunk {
                for word in line.replace([',', '.', ';'], "").to_lowercase().split(' ') {
                    *local_words.entry(word.to_string()).or_default() += 1;
                }
            }
            tx.send(local_words).unwrap();
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let mut words = HashMap::new();

    for thread_words in rx.try_iter() {
        for (word, count) in thread_words {
            *words.entry(word).or_insert(count) += count;
        }
    }

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
