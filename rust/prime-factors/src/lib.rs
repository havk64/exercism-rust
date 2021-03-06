fn is_prime(c: &u64, primes: &mut Vec<u64>) -> bool {
    if !primes.iter().any(|i| c % i == 0) {
        primes.push(*c);
        false
    } else {
        true
    }
}
pub fn factors(n: u64) -> Vec<u64> {
    let mut primes = Vec::new();
    let mut result = Vec::new();
    let mut x = n;

    let _ = (2..x)
        .filter(|c| is_prime(c, &mut primes))
        .collect::<Vec<_>>();
    while x > 0 {
        for p in primes.iter() {
            if x % p == 0 {
                x /= p;
                result.push(*p);
                break;
            }
        }
    }
    result
}
