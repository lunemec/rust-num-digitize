extern crate num;


use std::clone::Clone;
use std::ops::DivAssign;
use num::*;

/// Converts integer of type N (all implementations supported by num::Integer)
/// and returns a `Vec<N>` of its digits (base 10).
///
/// # Examples
///
/// ```
/// let number: u8 = 12;
/// println!("{:?}", digitize(number));
/// [1, 2]
/// ```
pub fn digitize<N: Copy + Clone + num::Integer + DivAssign>(number: N) -> Vec<N> {
    let mut number = number.clone();
    let mut digits: Vec<N> = Vec::new();

    let _0 = N::zero();
    let _1 = N::one();
    let _2 = _1 + _1;
    let _3 = _1 + _2;
    let _5 = _2 + _3;
    let _10 = _5 * _2;
    while number > _0 {
        let remainder = number % _10;
        digits.insert(0, remainder);
        number /= _10;
    }

    return digits;
}


#[test]
fn test_basic_number() {
    let number: i32 = 1234;
    let vector: Vec<i32> = vec![1, 2, 3, 4];
    assert!(digitize(number) == vector);
}

#[test]
fn test_zero() {
    let number = 0;
    assert!(digitize(number) == vec![]);
}

#[test]
fn test_u64() {
    let number: u64 = 12341234;
    let vector: Vec<u64> = vec![1, 2, 3, 4, 1, 2, 3, 4];
    assert!(digitize(number) == vector);
}

#[test]
fn test_usize() {
    let number: usize = 1234567890;
    let vector: Vec<usize> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
    assert!(digitize(number) == vector);
}
