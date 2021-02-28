pub fn square(s: u32) -> u64 {
    (1..s).map(|x| x.pow(2)).sum::<u64>()
}

pub fn total() -> u64 {
    unimplemented!();
}
