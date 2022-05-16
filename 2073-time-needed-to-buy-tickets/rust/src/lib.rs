struct Solution;

impl Solution {
    pub fn time_required_to_buy(mut tickets: Vec<i32>, k: i32) -> i32 {
        let mut result = 0;
        while tickets[k as usize] != 0 {
            for i in 0..tickets.len() {
                if tickets[i] > 0 {
                    tickets[i] -= 1;
                    result += 1;
                }
                if tickets[k as usize] == 0 {
                    break;
                }
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
        assert_eq!(Solution::time_required_to_buy(vec![2, 3, 2], 2), 6);
        assert_eq!(Solution::time_required_to_buy(vec![5, 1, 1, 1], 0), 8);
    }
}
