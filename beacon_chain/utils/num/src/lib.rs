use std::cmp::PartialOrd;

use num::{FromPrimitive, Integer};

/// Returns input such that it is no lower than min and no higher than max.
// NOTE: clipped from here: https://crates.io/crates/num
// num = "0.2.0"
// Tried re-exporting the function but led to some strange doc test errors
// If this (or related) function(s) become a maintenance overhead, try using the linked crate.
// TODO ask ralexstokes about status of this...
pub fn clamp<T: PartialOrd>(input: T, min: T, max: T) -> T {
    if input < min {
        min
    } else if input > max {
        max
    } else {
        input
    }
}

/// The largest integer `x` such that `x**2` is less than `input`.
pub fn integer_square_root<Z: Integer + FromPrimitive + Copy>(input: Z) -> Z {
    let one = FromPrimitive::from_u64(1).unwrap();
    let two = FromPrimitive::from_u64(2).unwrap();

    let mut x = input;
    let mut y = (x + one).div_floor(&two);

    while y < x {
        x = y;
        y = (x + input / x).div_floor(&two);
    }

    x
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn can_clamp() {
        let upper = 20;
        let lower = 10;
        for i in 0..25 {
            let bound = clamp(i, lower, upper);
            if i <= lower {
                assert_eq!(bound, lower);
            } else if i >= upper {
                assert_eq!(bound, upper);
            } else {
                assert!(
                    bound > lower && bound < upper,
                    "clamp failed; check `num` crate"
                )
            }
        }
    }

    #[test]
    fn can_take_int_sqrt() {
        let test = 20;
        let result = integer_square_root(test);
        assert_eq!(result, 4);
    }
}
