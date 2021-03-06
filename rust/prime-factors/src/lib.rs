pub fn factors(n: u64) -> Vec<u64> {
    let mut result = Vec::new();
    let mut x  = n;

    while x > 1 {
        for p in 2..n {
            if x % p == 0 {
                x /= p;
                result.push(p);
                break;
            }
        }
    }
    result
}
