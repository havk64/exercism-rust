fn decompose(mut num: u32) -> Vec<u32> {
    let mut digits: Vec<u32> = Vec::new();
    let mut power = 1;
    let mut n = num;

    while n > 9 {
        n /= 10;
        power *= 10;
    }

    while power > 0 {
        digits.push(num / power);
        num %= power;
        power /= 10;
    }
    digits
}
pub fn is_armstrong_number(num: u32) -> bool {
    let dec = decompose(num);
    let power = dec.len() as u32;

    dec.iter()
        .map(|&digit| digit.pow(power))
        .sum::<u32>() == num
}
