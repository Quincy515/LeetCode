#![allow(dead_code)]
struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn fib(n: i32) -> i32 {
        if n == 0 { return 0; }
        if n == 1 { return 1; }
        return (Self::fib(n - 1) + Self::fib(n - 2)) % 1000000007;
    }

    pub fn memo_fib(n: i32) -> i32 {
        let mut memo: HashMap<i32, i32> = HashMap::new();

        fn calculate(n: i32, memo: &mut HashMap<i32, i32>) -> i32 {
            let _mod = 1000000007;
            if n == 0 { return 0; }
            if n == 1 { return 1; }
            if memo.contains_key(&n) { return *memo.get(&n).unwrap(); }

            let ret = (calculate(n - 1, memo) + calculate(n - 2, memo)) % _mod;
            memo.insert(n, ret);
            return ret;
        }
        calculate(n, &mut memo)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {

        assert_eq!(3, Solution::memo_fib(4));
        assert_eq!(832040, Solution::memo_fib(30));
    }
}
