fn is_prime(n: u32) -> bool {
    !(2..n - 1).any(|i| n % i == 0)
}
pub fn nth(n: u32) -> u32 {
    match n {
        0 => 0,
        n => (1..)
            .filter(|c| -> bool { is_prime(*c) })
            .nth((n + 1) as usize)
            .unwrap(),
    }
}
