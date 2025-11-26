use std::collections::{HashMap, HashSet};

pub fn check2(candidate: &str) -> bool {
    let mut counts: HashMap<char, usize> = HashMap::new();

    for c in candidate.to_lowercase().chars() {
        if c.is_ascii_alphabetic() {
            *counts.entry(c).or_insert(0) += 1;
            if counts.get(&c) > Some(&1) {
                return false;
            }
        }
    }

    true
}

pub fn check(candidate: &str) -> bool {
    let mut seen = HashSet::new();
    candidate
        .to_lowercase()
        .chars()
        .filter(char::is_ascii_alphabetic)
        .all(|c| seen.insert(c))
}
