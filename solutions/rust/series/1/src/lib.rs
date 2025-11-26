pub fn series(digits: &str, len: usize) -> Vec<String> {
    digits
        .chars()
        .collect::<Vec<char>>()
        .windows(len)
        .map(|w| w.iter().collect()) // each slice of chars becomes a String
        .collect()
}
