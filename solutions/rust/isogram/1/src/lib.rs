use std::collections::HashMap;

pub fn check(candidate: &str) -> bool {
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
