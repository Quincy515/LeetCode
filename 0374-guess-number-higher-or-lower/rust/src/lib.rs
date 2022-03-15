#![allow(non_snake_case)]

/**
 * Forward declaration of guess API.
 * @param  num   your guess
 * @return 	     -1 if num is lower than the guess number
 *			      1 if num is higher than the guess number
 *               otherwise return 0
 * unsafe fn guess(num: i32) -> i32 {}
 */
struct Solution;

static mut X: i32 = 0;

unsafe fn guess(num: i32) -> i32 {
    use std::cmp::Ordering::*;
    match X.cmp(&num) {
        Equal => 0,
        Less => -1,
        Greater => 1,
    }
}

impl Solution {
    unsafe fn guessNumber(n: i32) -> i32 {
        let (mut left, mut right) = (1, n);
        while left < right {
            let m = left + (right - left) / 2;
            match guess(m) {
                0 => return m,
                -1 => right = m,
                1 => left = m + 1,
                _ => {}
            }
        }
        left
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        unsafe {
            super::X = 6;
            assert_eq!(super::Solution::guessNumber(10), 6);
        }
    }
}
