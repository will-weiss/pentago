extern crate num;

use self::num::traits::{Zero, One, ToPrimitive};
use self::num::bigint::BigUint;

pub fn mult2(x: BigUint) -> BigUint {
    x << 1
}

pub fn mult3(x: BigUint) -> BigUint {
    let mut _x = x.clone();
    _x = _x << 1;
    _x + x
}

pub fn three_raised_to(x: u32) -> BigUint {
    (0..x).fold(BigUint::one(), |exp, _| {
        mult3(exp)
    })
}
