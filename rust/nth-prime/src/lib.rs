fn is_prime(primes: &mut Vec<u32>, c: &u32) -> bool {
    if !primes.iter().any(|i| c % i == 0) {
        primes.push(*c);
        true
    } else {
        false
    }
}
pub fn nth(n: u32) -> u32 {
    let mut primes: Vec<u32> = Vec::new();

    (2..)
        .filter(|c| -> bool { is_prime(&mut primes, c) })
        .nth(n as usize)
        .unwrap()
}
