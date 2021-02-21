pub fn nth(n: u32) -> u32 {
    let mut primes: Vec<u32> = Vec::new();

    (2..)
        .filter(|c| -> bool {
            if !primes.iter().any(|i| c % i == 0) {
                primes.push(*c);
                true
            } else {
                false
            }
        })
        .nth(n  as usize)
        .unwrap()
}
