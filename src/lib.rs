#![doc(html_root_url = "https://lunemec.github.io/rust-num-digitize/")]
extern crate num;

use std::ops::DivAssign;
use num::*;

/// Converts integer of type `N` (all implementations supported by `num::Integer`)
/// and returns a `Vec<u8>` of its digits (base 10).
///
/// # Arguments
///
/// * `number` - any number type which have implementation for `num::Integer`
///
/// # Example
///
/// Basic usage:
///
/// ```
/// use num_digitize::digitize;
///
/// let number: u8 = 12;
/// let vector: Vec<u8> = vec![1, 2];
/// assert!(digitize(number) == vector);
/// ```
///
/// Negative numbers return empty `Vec<u8>`:
///
/// ```
/// use num_digitize::digitize;
/// let number = -12;
/// assert!(digitize(number) == vec![]);
/// ```
pub fn digitize<N: Copy + Clone + Integer + NumCast + DivAssign>(number: N) -> Vec<u8> {
    let mut number = number.clone();
    let mut digits: Vec<u8> = Vec::new();

    let _0 = N::zero();
    let _10 = {
        let _1 = N::one();
        let _2 = _1 + _1;
        let _3 = _1 + _2;
        let _5 = _2 + _3;
        _5 * _2
    };
    while number > _0 {
        let remainder: u8 = cast(number % _10).unwrap();
        digits.insert(0, remainder);
        number /= _10;
    }

    digits
}


#[test]
fn test_basic_number() {
    let number: i32 = 1234;
    let vector: Vec<u8> = vec![1, 2, 3, 4];
    assert!(digitize(number) == vector);
}

#[test]
fn test_zero() {
    let number = 0;
    let vector: Vec<u8> = vec![];
    assert!(digitize(number) == vector);
}

#[test]
fn test_u64() {
    let number: u64 = 12341234;
    let vector: Vec<u8> = vec![1, 2, 3, 4, 1, 2, 3, 4];
    assert!(digitize(number) == vector);
}

#[test]
fn test_usize() {
    let number: usize = 1234567890;
    let vector: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
    assert!(digitize(number) == vector);
}

#[test]
fn test_negative() {
    let number = -1234567890;
    let vector: Vec<u8> = vec![];
    assert!(digitize(number) == vector);
}
