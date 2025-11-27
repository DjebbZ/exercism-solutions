pub fn egg_count(display_value: u32) -> usize {
    let mut current = display_value;
    let mut result = 0;

    while current > 0 {
        let digit = current % 2;
        if digit == 1 {
            result += 1;
        }
        current /= 2;
    }

    result
}
