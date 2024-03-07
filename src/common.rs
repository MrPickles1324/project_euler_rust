use num_bigint::BigUint;

pub fn is_prime(n: usize) -> bool {
    if n == 2 || n == 3 {
        return true;
    }

    if n <= 1 || n % 2 == 0 || n % 3 == 0 {
        return false;
    }

    for i in (5..).step_by(6).take_while(|i| i * i <= n) {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
    }

    true
}

pub fn factorial<T: Into<BigUint>>(n: T) -> BigUint {
    num_iter::range_inclusive(1u32.into(), n.into()).product()
}
