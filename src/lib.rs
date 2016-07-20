#![doc(html_root_url = "https://lunemec.github.io/rust-num-digitize/")]
extern crate num;

use num::cast;
//use num_traits::cast::NumCast;
//use num_traits::int::PrimInt;

pub trait ToDigits {
    /// Converts integer to `Vec<i8>` of its digits (base 10).
    ///
    /// # Example
    ///
    /// Basic usage:
    ///
    /// ```
    /// use num_digitize::ToDigits;
    ///
    /// let number: u8 = 12;
    /// let vector: Vec<i8> = vec![1, 2];
    /// assert!(number.to_digits() == vector);
    /// ```
    ///
    /// Negative numbers return all the digits negative `Vec<u8>`:
    ///
    /// ```
    /// use num_digitize::ToDigits;
    /// let number = -12;
    /// assert!(number.to_digits() == vec![-1, -2]);
    /// ```
    ///
    /// Reason for this is mathematically you can easily add these back to the original number like so:
    ///
    /// ```text
    /// -123 -> [-1, -2, -3]
    /// (-1 * 10^2) + (-2 * 10^1) + (-3 * 10^0) = -123
    /// ```
    ///
    /// Or with `FromDigits` trait.
    fn to_digits(&self) -> Vec<i8>;
}

pub trait FromDigits {
    /// Converts `Vec<N>` of digits back to the original number.
    ///
    /// # Example
    ///
    /// Basic usage:
    ///
    /// ```
    /// use num_digitize::FromDigits;
    ///
    /// let vector = vec![1, 2, 3, 4, 5];
    /// let number = 12345;
    /// assert!(vector.from_digits() == number);
    /// ```
    ///
    /// Negative numbers:
    ///
    /// ```
    /// use num_digitize::FromDigits;
    ///
    /// let vector = vec![-1, -2, -3, -4, -5];
    /// let number = -12345;
    /// assert!(vector.from_digits() == number);
    /// ```
    ///
    /// It works like this:
    /// ```text
    /// [-1, -2, -3]
    /// (-1 * 10^2) + (-2 * 10^1) + (-3 * 10^0) = -123
    /// ```
    ///
    /// Please note, all of the digits have to be negative in order to get the correct number back.
    ///
    /// Also note, if you use this on vector of larger numbers (> 9 or < -9), the results will be wrong.
    ///
    /// See `ToDigits` trait for details.
    fn from_digits(&self) -> isize;
}

macro_rules! impl_for {
    ($IntegerType: ty) => {
        impl ToDigits for $IntegerType {
            fn to_digits(&self) -> Vec<i8> {
                let mut number = self.clone();
                let mut digits: Vec<i8> = Vec::new();

                while number != 0 {
                    let remainder: i8 = cast(number % 10).unwrap();
                    digits.insert(0, remainder);
                    number /= 10;
                }

                digits
            }
        }

        impl FromDigits for Vec<$IntegerType> {
            fn from_digits(&self) -> isize {
                let ten: isize = 10;

                self.into_iter().rev().enumerate().fold(
                    0isize,
                    |mut sum, (i, number)| {sum += *number as isize * ten.pow(i as u32); sum}
                )
            }
        }
    }
}

impl_for! {i8}
impl_for! {i16}
impl_for! {i32}
impl_for! {i64}
impl_for! {usize}
impl_for! {u8}
impl_for! {u16}
impl_for! {u32}
impl_for! {u64}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_to_zero() {
        let i = 0;
        let vector: Vec<i8> = vec![];
        assert!(i.to_digits() == vector);
    }

    #[test]
    fn test_from_zero() {
        let i = 0;
        let vector: Vec<i8> = vec![];
        assert!(vector.from_digits() == i);
    }

    #[test]
    fn test_to_digits_i8() {
        let i: i8 = 123;
        let vector: Vec<i8> = vec![1, 2, 3];
        assert!(i.to_digits() == vector);
    }

    #[test]
    fn test_from_digits_i8() {
        let i = 123;
        let vector: Vec<i8> = vec![1, 2, 3];
        assert!(vector.from_digits() == i);
    }

    #[test]
    fn test_to_digits_i8_negative() {
        let i: i8 = -56;
        let vector: Vec<i8> = vec![-5, -6];
        assert!(i.to_digits() == vector);
    }

    #[test]
    fn test_from_digits_i8_negative() {
        let i = -56;
        let vector: Vec<i8> = vec![-5, -6];
        assert!(vector.from_digits() == i);
    }

    #[test]
    fn test_to_digits_i16() {
        let i: i16 = 12366;
        let vector: Vec<i8> = vec![1, 2, 3, 6, 6];
        assert!(i.to_digits() == vector);
    }

    #[test]
    fn test_from_digits_i16() {
        let i = 12366;
        let vector: Vec<i16> = vec![1, 2, 3, 6, 6];
        assert!(vector.from_digits() == i);
    }

    #[test]
    fn test_to_digits_i32() {
        let i: i32 = 4555;
        let vector: Vec<i8> = vec![4, 5, 5, 5];
        assert!(i.to_digits() == vector);
    }

    #[test]
    fn test_from_digits_i32() {
        let i = 4555;
        let vector: Vec<i32> = vec![4, 5, 5, 5];
        assert!(vector.from_digits() == i);
    }

    #[test]
    fn test_to_digits_i64() {
        let i: i64 = 45559;
        let vector: Vec<i8> = vec![4, 5, 5, 5, 9];
        assert!(i.to_digits() == vector);
    }

    #[test]
    fn test_from_digits_i64() {
        let i = 45559;
        let vector: Vec<i64> = vec![4, 5, 5, 5, 9];
        assert!(vector.from_digits() == i);
    }

    #[test]
    fn test_to_digits_u8() {
        let i: u8 = 255;
        let vector: Vec<i8> = vec![2, 5, 5];
        assert!(i.to_digits() == vector);
    }

    #[test]
    fn test_from_digits_u8() {
        let i = 255;
        let vector: Vec<u8> = vec![2, 5, 5];
        assert!(vector.from_digits() == i);
    }

    #[test]
    fn test_to_digits_u16() {
        let i: u16 = 25945;
        let vector: Vec<i8> = vec![2, 5, 9, 4, 5];
        assert!(i.to_digits() == vector);
    }

    #[test]
    fn test_from_digits_u16() {
        let i = 25945;
        let vector: Vec<u16> = vec![2, 5, 9, 4, 5];
        assert!(vector.from_digits() == i);
    }

    #[test]
    fn test_to_digits_u32() {
        let i: u32 = 2591222345;
        let vector: Vec<i8> = vec![2, 5, 9, 1, 2, 2, 2, 3, 4, 5];
        assert!(i.to_digits() == vector);
    }

    #[test]
    fn test_from_digits_u32() {
        let i = 2591222345;
        let vector: Vec<u32> = vec![2, 5, 9, 1, 2, 2, 2, 3, 4, 5];
        assert!(vector.from_digits() == i);
    }

    #[test]
    fn test_to_digits_u64() {
        let i: u64 = 12341234765432;
        let vector: Vec<i8> = vec![1, 2, 3, 4, 1, 2, 3, 4, 7, 6, 5, 4, 3, 2];
        assert!(i.to_digits() == vector);
    }

    #[test]
    fn test_from_digits_u64() {
        let i = 12341234765432;
        let vector: Vec<i8> = vec![1, 2, 3, 4, 1, 2, 3, 4, 7, 6, 5, 4, 3, 2];
        assert!(vector.from_digits() == i);
    }

    #[test]
    fn test_to_usize() {
        let i: usize = 1234567890;
        let vector: Vec<i8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
        assert!(i.to_digits() == vector);
    }

    #[test]
    fn test_from_usize() {
        let i:isize = 1234567890;
        let vector: Vec<i8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
        assert!(vector.from_digits() == i);
    }
}