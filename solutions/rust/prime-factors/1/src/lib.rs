pub fn factors(n: u64) -> Vec<u64> {

    let mut prime_factors: Vec<u64> = Vec::new();
    let mut given_number = n;

    let mut i = 2;
    while given_number > 1 {
        if given_number % i == 0 {
            prime_factors.push(i);
            given_number /= i;
        } else {
            i += 1;
        }
    }

    prime_factors
}

/// https://en.wikipedia.org/wiki/Primality_test
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