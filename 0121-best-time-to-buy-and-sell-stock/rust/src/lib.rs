struct Solution;
impl Solution {
    pub fn max_profit_v1(prices: Vec<i32>) -> i32 {
        let (n, mut max, mut cur_max) = (prices.len(), vec![0; prices.len()], 0);
        for i in (0..=n - 1).rev() {
            max[i] = cur_max;
            if prices[i] > cur_max {
                cur_max = prices[i];
            }
        }
        let mut result = 0;
        for i in 0..n {
            if result < max[i] - prices[i] {
                result = max[i] - prices[i];
            }
        }
        result
    }

    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut min = prices[0];

        for p in prices {
            max = max.max(p - min);
            min = min.min(p);
        }

        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
        assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
    }
}
