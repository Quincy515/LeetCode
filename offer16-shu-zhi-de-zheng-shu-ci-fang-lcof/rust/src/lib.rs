#![allow(dead_code)]
struct Solution;
impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        if n >= 0 {
            return Self::r_pow(x, n);
        } else {
            return 1 as f64 / (Self::r_pow(x, -1 * (n + 1)) * x);
        }
    }

    fn r_pow(x: f64, n: i32) -> f64 {
        if n == 0 {
            return 1 as f64;
        }
        let half_pow = Self::r_pow(x, n / 2);
        if n % 2 == 1 {
            return half_pow * half_pow * x;
        } else {
            return half_pow * half_pow;
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
