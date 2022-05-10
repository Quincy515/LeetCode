impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let (mut left, mut right) = (1, *piles.iter().max().unwrap());

        while left < right {
            let mid = left + (right - left) / 2;
            let mut sum = 0;
            for p in &piles {
                sum += if p % mid == 0 { p / mid } else { p / mid + 1 };
            }
            if sum > h {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        left
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn it_works() {
        assert_eq!(Solution::min_eating_speed(vec![3, 6, 7, 11], 8), 4);
        assert_eq!(Solution::min_eating_speed(vec![30, 11, 23, 4, 20], 5), 30);
        assert_eq!(Solution::min_eating_speed(vec![30, 11, 23, 4, 20], 6), 23);
    }
}
