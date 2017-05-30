#![doc(html_root_url = "https://lunemec.github.io/rust-num-digitize/")]
extern crate num;

use std::iter::IntoIterator;
use std::ops::DivAssign;
use num::{Num, NumCast, cast};


pub trait ToDigits
where Self: Copy + Clone + Num + NumCast + DivAssign {
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
    /// assert_eq!(number.to_digits(), vector);
    /// ```
    ///
    /// Negative numbers return all the digits negative `Vec<u8>`:
    ///
    /// ```
    /// use num_digitize::ToDigits;
    /// let number = -12;
    /// assert_eq!(number.to_digits(), vec![-1, -2]);
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
    fn to_digits(&self) -> Vec<i8> {
        let mut number = self.clone();
        let mut digits: Vec<i8> = Vec::new();

        let zero: Self = cast(0).unwrap();
        let ten: Self = cast(10).unwrap();

        while number != zero {
            let remainder: i8 = cast(number % ten).unwrap();
            digits.insert(0, remainder);
            number /= ten;
        }

        digits
    }
}

pub trait FromIterRadix  // Thank you *insaneinside*!
where Self: Copy + Sized + Num + NumCast {
    /// Converts a iterable of numbers back to original number of given base.
    ///
    /// # Example
    ///
    /// Basic usage:
    ///
    /// ```
    /// use num_digitize::FromIterRadix;
    ///
    /// let original_number = 12345678;
    /// let output = i8::from_iter_radix([1, 2, 3, 4, 5, 6, 7, 8].iter().cloned(), 10);
    /// assert_eq!(original_number, output);
    /// ```
    ///
    /// Please note, bases other than 10 were not tested and may not give any sense!
    fn from_iter_radix<I: IntoIterator<Item=Self>>(iter: I, base: i64) -> i64 {
        iter.into_iter().fold(
            0i64,
            |mut sum: i64, number| {sum *= base; sum += cast::<Self,i64>(number).unwrap(); sum}
        )
    }
}

pub trait FromDigits<N>
where N: FromIterRadix, Self: Sized + IntoIterator<Item=N> {
    /// Converts iterable of digits back to the original number.
    /// This does not currently work for slices.
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
    /// assert_eq!(vector.from_digits(), number);
    /// ```
    ///
    /// Negative numbers:
    ///
    /// ```
    /// use num_digitize::FromDigits;
    ///
    /// let vector = vec![-1, -2, -3, -4, -5];
    /// let number = -12345;
    /// assert_eq!(vector.from_digits(), number);
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
    /// Also note, if you use this on vector of larger numbers (> 9 or < -9), the results will be
    /// wrong.
    ///
    /// See `ToDigits` trait for details.
    fn from_digits(self) -> i64 {
        N::from_iter_radix(self, 10i64)
    }
}

impl ToDigits for i8 {}
impl ToDigits for i16 {}
impl ToDigits for i32 {}
impl ToDigits for i64 {}
impl ToDigits for isize {}
impl ToDigits for u8 {}
impl ToDigits for u16 {}
impl ToDigits for u32 {}
impl ToDigits for u64 {}
impl ToDigits for usize {}

impl FromIterRadix for i8 {}
impl FromIterRadix for i16 {}
impl FromIterRadix for i32 {}
impl FromIterRadix for i64 {}
impl FromIterRadix for isize {}
impl FromIterRadix for u8 {}
impl FromIterRadix for u16 {}
impl FromIterRadix for u32 {}
impl FromIterRadix for u64 {}
impl FromIterRadix for usize {}

impl<I, N> FromDigits<N> for I where N: FromIterRadix, I: IntoIterator<Item=N> {}

#[cfg(test)]
mod to_digits {
    use super::ToDigits;

    #[test]
    fn test_zero() {
        let i = 0;
        let vector: Vec<i8> = vec![];
        assert_eq!(i.to_digits(), vector);
    }

    #[test]
    fn test_i8() {
        let i: i8 = 123;
        let vector: Vec<i8> = vec![1, 2, 3];
        assert_eq!(i.to_digits(), vector);
    }

    #[test]
    fn test_i8_negative() {
        let i: i8 = -56;
        let vector: Vec<i8> = vec![-5, -6];
        assert_eq!(i.to_digits(), vector);
    }

    #[test]
    fn test_i16() {
        let i: i16 = 12366;
        let vector: Vec<i8> = vec![1, 2, 3, 6, 6];
        assert_eq!(i.to_digits(), vector);
    }

    #[test]
    fn test_i32() {
        let i: i32 = 4555;
        let vector: Vec<i8> = vec![4, 5, 5, 5];
        assert_eq!(i.to_digits(), vector);
    }

    #[test]
    fn test_i64() {
        let i: i64 = 45559;
        let vector: Vec<i8> = vec![4, 5, 5, 5, 9];
        assert_eq!(i.to_digits(), vector);
    }

    #[test]
    fn test_isize() {
        let i: isize = 4555934;
        let vector: Vec<i8> = vec![4, 5, 5, 5, 9, 3, 4];
        assert_eq!(i.to_digits(), vector);
    }

    #[test]
    fn test_u8() {
        let i: u8 = 255;
        let vector: Vec<i8> = vec![2, 5, 5];
        assert_eq!(i.to_digits(), vector);
    }

    #[test]
    fn test_u16() {
        let i: u16 = 25945;
        let vector: Vec<i8> = vec![2, 5, 9, 4, 5];
        assert_eq!(i.to_digits(), vector);
    }

    #[test]
    fn test_u32() {
        let i: u32 = 2591222345;
        let vector: Vec<i8> = vec![2, 5, 9, 1, 2, 2, 2, 3, 4, 5];
        assert_eq!(i.to_digits(), vector);
    }

    #[test]
    fn test_u64() {
        let i: u64 = 12341234765432;
        let vector: Vec<i8> = vec![1, 2, 3, 4, 1, 2, 3, 4, 7, 6, 5, 4, 3, 2];
        assert_eq!(i.to_digits(), vector);
    }

    #[test]
    fn test_usize() {
        let i: usize = 1234567890;
        let vector: Vec<i8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
        assert_eq!(i.to_digits(), vector);
    }

}

#[cfg(test)]
mod from_iter_radix {
    use super::FromIterRadix;

    #[test]
    fn test_vector_i8() {
        let i = 12345678;
        let result = i8::from_iter_radix(vec![1, 2, 3, 4, 5, 6, 7, 8], 10);
        assert_eq!(i, result);
    }

    #[test]
    fn test_slice_i8() {
        let i = 12345678;
        let result = i8::from_iter_radix([1, 2, 3, 4, 5, 6, 7, 8].iter().cloned(), 10);
        assert_eq!(i, result);
    }
}

#[cfg(test)]
mod from_digits {
    use super::FromDigits;

    #[test]
    fn test_zero() {
        let i = 0;
        let vector: Vec<i8> = vec![];
        assert_eq!(vector.from_digits(), i);
    }

    #[test]
    fn test_multiple_zeros() {
        let i = 0;
        let vector: Vec<i8> = vec![0, 0, 0, 0, 0, 0, 0];
        assert_eq!(vector.from_digits(), i);
    }

    #[test]
    fn test_i8() {
        let i = 123;
        let vector: Vec<i8> = vec![1, 2, 3];
        assert_eq!(vector.from_digits(), i);
    }

    #[test]
    fn test_i8_negative() {
        let i = -56;
        let vector: Vec<i8> = vec![-5, -6];
        assert_eq!(vector.from_digits(), i);
    }

    #[test]
    fn test_i16() {
        let i = 12366;
        let vector: Vec<i16> = vec![1, 2, 3, 6, 6];
        assert_eq!(vector.from_digits(), i);
    }

    #[test]
    fn test_i32() {
        let i = 4555;
        let vector: Vec<i32> = vec![4, 5, 5, 5];
        assert_eq!(vector.from_digits(), i);
    }

    #[test]
    fn test_i64() {
        let i = 45559;
        let vector: Vec<i64> = vec![4, 5, 5, 5, 9];
        assert_eq!(vector.from_digits(), i);
    }

    #[test]
    fn test_u8() {
        let i = 255;
        let vector: Vec<u8> = vec![2, 5, 5];
        assert_eq!(vector.from_digits(), i);
    }

    #[test]
    fn test_isize() {
        let i = 455597;
        let vector: Vec<isize> = vec![4, 5, 5, 5, 9, 7];
        assert_eq!(vector.from_digits(), i);
    }

    #[test]
    fn test_u16() {
        let i = 25945;
        let vector: Vec<u16> = vec![2, 5, 9, 4, 5];
        assert_eq!(vector.from_digits(), i);
    }

    #[test]
    fn test_u32() {
        let i = 2591222345;
        let vector: Vec<u32> = vec![2, 5, 9, 1, 2, 2, 2, 3, 4, 5];
        assert_eq!(vector.from_digits(), i);
    }

    #[test]
    fn test_u64() {
        let i = 12341234765432;
        let vector: Vec<u64> = vec![1, 2, 3, 4, 1, 2, 3, 4, 7, 6, 5, 4, 3, 2];
        assert_eq!(vector.from_digits(), i);
    }

    #[test]
    fn test_usize() {
        let i = 1234567890;
        let vector: Vec<usize> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
        assert_eq!(vector.from_digits(), i);
    }

    #[test]
    fn test_negative() {
        let i = -25945;
        let vector = vec![-2, -5, -9, -4, -5];
        assert_eq!(vector.from_digits(), i);
    }
}