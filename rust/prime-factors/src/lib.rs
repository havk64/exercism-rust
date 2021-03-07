pub fn factors(mut n: u64) -> Vec<u64> {
    let mut result = Vec::new();
    let mut range = 2..;

    while n > 1 {
        if let Some(c) = range.next() {
            while n % c == 0 {
                n /= c;
                result.push(c);
            }
        }
    }
    result
}
