pub fn nth(n: u32) -> u32 {
    let mut primes: Vec<u32> = Vec::new();
    let mut is_prime = true;

    for x in 2..u32::MAX  {
        for y in primes[..].iter() {
            if x % y == 0 {
                is_prime = false;
                break;
            }
        }
        if is_prime {
            primes.push(x);
        }
        if primes.len() == (n + 1) as usize {
            return x;
        }
        is_prime = true;
    }
    0
}
