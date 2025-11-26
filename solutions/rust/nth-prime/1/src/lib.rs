/// Returns the nth prime number
pub fn nth(n: u32) -> u32 {
    let mut primes: Vec<u32> = Vec::new();

    for i in 0.. {
        if primes.len() > n as usize {
            break;
        }
        if is_prime(i) {
            primes.push(i);
        }
    }
    primes[n as usize]
}

/// Primality test from Wikipedia: https://en.wikipedia.org/wiki/Primality_test
fn is_prime(n: u32) -> bool {
    if n == 2 || n == 3 {
        return true;
    }

    if n <= 1 || n % 2 == 0 || n % 3 == 0 {
        return false;
    }

    for i in (5..).step_by(6).take_while(|i| i * i <= n) {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
    }

    true
}
