pub fn collatz(n: u64) -> Option<u64> {
    // todo!("return Some(x) where x is the number of steps required to reach 1 starting with {n}")
    let mut n = n;
    for steps in 0.. {
        match n {
            0 => return None,
            1 => return Some(steps),
            even if even % 2 == 0 => n /= 2,
            _ => n = n.checked_mul(3)?.checked_add(1)?,
        }
    }

    None
}
