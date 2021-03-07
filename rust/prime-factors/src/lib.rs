pub fn factors(mut n: u64) -> Vec<u64> {
    let mut result = Vec::new();
    let mut c  = 2..;

    while n > 1 {
        let c = c.next().unwrap();

        while n % c == 0 {
            n /= c;
            result.push(c);
        }
    }
    result
}
