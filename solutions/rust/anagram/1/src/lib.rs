use std::collections::{HashMap, HashSet};

trait Freq {
    fn frequencies(&self) -> HashMap<String, usize>;
}

impl Freq for str {
    fn frequencies(&self) -> HashMap<String, usize> {
        let mut freq = HashMap::new();
        for c in self.chars() {
            *freq.entry(c.to_lowercase().to_string()).or_insert(0) += 1;
        }
        freq
    }
}

#[cfg(test)]
mod tests {
    use crate::Freq;

    #[test]
    fn test_frequencies() {
        let s = "hello";
        let freq = s.frequencies();
        assert_eq!(freq.get(&'h'.to_string()).unwrap(), &1);
        assert_eq!(freq.get(&'e'.to_string()).unwrap(), &1);
        assert_eq!(freq.get(&'l'.to_string()).unwrap(), &2);
        assert_eq!(freq.get(&'o'.to_string()).unwrap(), &1);
    }
}


pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word_freq = word.frequencies();
    let possible_anagrams: HashSet<&str> = possible_anagrams
        .iter()
        .filter(|w| { dbg!(w, word); w.to_lowercase() != word.to_lowercase() })
        .filter(|w| { dbg!(w.frequencies(), &word_freq); w.frequencies() == word_freq })
        .map(|w| *w)
        .collect();
    dbg!(possible_anagrams)
}
