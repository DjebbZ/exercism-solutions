use rand::Rng;

/// Pick a private key greater than 1 and less than {p}
pub fn private_key(prime1: u64) -> u64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(2..prime1)
}

pub fn public_key(prime1: u64, prime2: u64, private_key: u64) -> u64 {
    modular_exponentiation(prime2, private_key, prime1)
}

pub fn secret(prime1: u64, b_pub: u64, private_key: u64) -> u64 {
    modular_exponentiation(b_pub, private_key, prime1)
}

/// Calculate the modular exponentiation based on the "Right-to-Left binary method" of https://en.wikipedia.org/wiki/Modular_exponentiation
pub fn modular_exponentiation(mut base: u64, mut exponent: u64, modulus: u64) -> u64 {
    if modulus == 1 {
        return 0;
    }
    let mut result = 1;
    base = base % modulus;
    while exponent > 0 {
        if exponent % 2 == 1 {
            result = (result * base) % modulus;
        }
        exponent = exponent >> 1;
        base = (base * base) % modulus;
    }
    result
}

#[test]
fn test_modular_exponentiation_wikipedia() {
    assert_eq!(modular_exponentiation(4, 13, 497), 445);
}