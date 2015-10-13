extern crate num;

use self::num::traits::One;
use self::num::bigint::BigUint;

pub fn mult2(x: BigUint) -> BigUint {
    x << 1
}

pub fn mult3(x: BigUint) -> BigUint {
    let mut _x = x.clone();
    _x = _x << 1;
    _x + x
}

pub fn three_raised_to(x: usize) -> BigUint {
    (0..x).fold(BigUint::one(), |exp, _| {
        mult3(exp)
    })
}

pub fn factorial(num: usize) -> usize {
    (0..num).fold(1, |fact, n| {
        fact * n
    })
}
