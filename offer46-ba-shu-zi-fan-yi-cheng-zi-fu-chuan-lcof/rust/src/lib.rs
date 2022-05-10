struct Solution;

impl Solution {
    pub fn translate_num(num: i32) -> i32 {
        if num >= 10 {
            (if 10 <= num % 100 && num % 100 <= 25 {
                Solution::translate_num(num / 100)
            } else {
                0
            }) + Solution::translate_num(num / 10)
        } else {
            1
        }
    }

    pub fn translate_num2(mut num: i32) -> i32 {
        if num < 9 {
            return 1;
        }
        // 把十进制转化成数字数组
        let mut digitlist = vec![];
        while num != 0 {
            digitlist.push(num % 10);
            num /= 10;
        }
        let n = digitlist.len();
        let mut digits = vec![0; n];
        for i in 0..n {
            digits[i] = digitlist[n - i - 1];
        }

        let mut dp = vec![0; n + 1];
        dp[0] = 1;
        // dp[i] 表示 digits[0~i-1]（长度为 i）转化为字母有多少种方法
        // dp[i] = dp[i-1] + dp[i-2]（digits[i-2, i-1] 可翻译）
        // dp[i] = dp[i-1] (digits[i-2, i-1] 不可翻译)

        for i in 1..=n {
            dp[i] = dp[i - 1];
            if i >= 2 && Self::is_valid(digits[i - 2], digits[i - 1]) {
                dp[i] += dp[i - 2];
            }
        }

        dp[n]
    }

    fn is_valid(a: i32, b: i32) -> bool {
        if a == 1 {
            return true;
        }
        if a == 2 && b >= 0 && b <= 5 {
            return true;
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(5, Solution::translate_num(12258));
        assert_eq!(5, Solution::translate_num2(12258));
    }
}
