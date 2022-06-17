use rayon::prelude::*;
use std::collections::HashMap;
use std::sync::Mutex;

pub fn count_words(text: String, chunk_size: usize) -> Vec<(String, u32)> {
    let words: Mutex<HashMap<String, u32>> = Mutex::new(HashMap::new());

    let lines: Vec<_> = text.lines().collect();

    lines.par_chunks(chunk_size).for_each(|chunk| {
        let owned_chunk = chunk.to_vec();

        let mut local_words: HashMap<String, u32> = HashMap::new();
        for line in owned_chunk {
            for word in line
                .replace([',', '.', ';', ':', '!', '?', '(', ')', '"', '\''], "")
                .to_lowercase()
                .split(' ')
                .map(|w| w.trim())
                .filter(|w| !w.is_empty())
            {
                *local_words.entry(word.to_string()).or_insert(0) += 1;
            }
        }
        let mut words = words.lock().unwrap();
        for (word, count) in local_words {
            *words.entry(word).or_insert(0) += count;
        }
    });

    let words = words.into_inner().unwrap();

    let mut word_list = words.into_iter().collect::<Vec<_>>();
    word_list.par_sort_by_key(|e| e.1);
    word_list.reverse();

    word_list
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_words() {
        let s = "test".to_string();
        let res = count_words(s, 1);
        assert_eq!(res[0], ("test".to_string(), 1))
    }
}
