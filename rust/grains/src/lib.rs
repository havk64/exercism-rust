pub fn square(s: u32) -> u64 {
    let mut acc = 1;
    match s {
        1 => s as u64,
        2..=64 => {
            for _ in 1..s {
                acc *= 2
            }
            acc
        },
        _ => panic!("Square must be between 1 and 64"),
    }
}

pub fn total() -> u64 {
    (1..65).map(square).sum()
}
