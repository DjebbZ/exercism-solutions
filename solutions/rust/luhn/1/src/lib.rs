/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let code: String = code.chars().filter(|c| *c != ' ').collect();
    if code.len() <= 1 {
        return false;
    }
    if code.chars().any(|c| !c.is_ascii_digit()) {
        return false;
    }
    let sum: u32 = code.chars().filter(|c| *c != ' ').rev().enumerate().map(|(i, c)| {
        dbg!(i,c);
        if i % 2 == 0 {
            return c.to_digit(10).unwrap();
        }
        let sum = c.to_digit(10).unwrap() * 2;
        dbg!(sum);

        if sum > 9 {
            sum - 9
        } else {
            sum
        }
    }).sum();
    dbg!(sum, sum % 10);
    sum % 10 == 0
}
