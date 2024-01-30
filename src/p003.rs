use crate::common::is_prime;

pub fn solution() {
    let yep = (0..600851475143f64.sqrt().trunc() as usize)
        .filter(|i| is_prime(*i))
        .filter(|i| 600851475143 % i == 0)
        .last()
        .unwrap();
    println!("p003: {yep}");
}
