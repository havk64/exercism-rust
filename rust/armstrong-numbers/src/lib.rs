fn decompose(mut num: u32, base: u32) -> Vec<u32> {
    let mut digits: Vec<u32> = Vec::new();

    while num > 0 {
        digits.push(num % base);
        num /= 10;
    }
    digits
}
pub fn is_armstrong_number(num: u32) -> bool {
    let dec = decompose(num, 10);
    let power = dec.len() as u32;

    dec.iter()
        .map(|&digit| digit.pow(power))
        .sum::<u32>() == num
}
