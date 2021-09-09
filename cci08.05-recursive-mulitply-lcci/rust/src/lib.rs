#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn multiply(a: i32, b: i32) -> i32 {
        if a == 1 { return b; }
        let half_value = Self::multiply(a >> 1, b);
        if a % 2 == 1 { return half_value + half_value + b; }
        else { return half_value + half_value; }
    }
    pub fn optimization_multiply(a: i32, b: i32) -> i32 {
        let n = i32::min(a, b);
        let k = i32::max(a, b);
        if n == 1 { return k; }
        // n 个 k 相加 =(n/2 个 k 相加) + (n/2 个 k 相加) + 0(或k)
        let half_value = Self::optimization_multiply(n/2,k);
        if n % 2 == 1 { return half_value + half_value + k; }
        else { return half_value + half_value; }
    }
}