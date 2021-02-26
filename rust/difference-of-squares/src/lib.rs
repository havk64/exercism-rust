// We need to make the trait available to use its method, .sub() in this case
use std::ops::Sub;

pub fn square_of_sum(n: u32) -> u32 {
    (1..n+1).sum::<u32>().pow(2)
}

pub fn sum_of_squares(n: u32) -> u32 {
    (1..n+1).map(|x| x.pow(2)).sum::<u32>()
}

pub fn difference(n: u32) -> u32 {
    // Using the 'sub' method from the std::ops::Sub trait:
    square_of_sum(n).sub(sum_of_squares(n))
}
