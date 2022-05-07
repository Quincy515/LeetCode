struct Solution;

impl Solution {
    pub fn car_pooling(trips: Vec<Vec<i32>>, capacity: i32) -> bool {
        let mut nums = vec![0; 1002];
        for i in 0..trips.len() {
            let cnt = trips[i][0];
            let from = trips[i][1];
            let to = trips[i][2];
            nums[from as usize] += cnt;
            nums[to as usize] -= cnt;
        }
        let mut sum = 0;
        for i in 0..nums.len() {
            sum += nums[i];
            if sum > capacity {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert!(!Solution::car_pooling(
            vec![vec![2, 1, 5], vec![3, 3, 7]],
            4
        ));
        assert!(Solution::car_pooling(vec![vec![2, 1, 5], vec![3, 3, 7]], 5));
    }
}
