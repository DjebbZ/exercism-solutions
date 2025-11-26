pub fn is_armstrong_number(num: u32) -> bool {
    let digits: Vec<u32> = num
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    digits.iter().map(|d| (*d as u64).pow(digits.len() as u32)).sum::<u64>() == num as u64
}
