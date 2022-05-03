struct Solution;

impl Solution {
    pub fn minimum_cost(mut cost: Vec<i32>) -> i32 {
        cost.sort();
        let n = cost.len();
        if n == 1 {
            return cost[0];
        }

        let mut result = 0;

        for i in (0..n).rev().step_by(3) {
            if i < 1 {
                result += cost[i];
            } else {
                result += cost[i] + cost[i - 1];
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::minimum_cost(vec![1, 2, 3]), 5);
        assert_eq!(Solution::minimum_cost(vec![6, 5, 7, 9, 2, 2]), 23);
        assert_eq!(Solution::minimum_cost(vec![5, 5]), 10);
    }
}
