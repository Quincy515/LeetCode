struct Solution;

impl Solution {
    pub fn max_distance(mut position: Vec<i32>, m: i32) -> i32 {
        let mut result = -1;
        if position.is_empty() {
            return result;
        }
        position.sort_unstable();

        let (mut left, mut right) = (0, 10e9 as i32);
        while left < right {
            let mid = left + (right - left) / 2;
            if Self::check(mid as i32, &position, m) {
                result = mid as i32;
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        result
    }

    fn check(dis: i32, position: &[i32], mut m: i32) -> bool {
        let mut pre = position[0];
        m -= 1;
        for n in position.iter().skip(1) {
            if n - pre >= dis {
                pre = *n;
                m -= 1;
                if m == 0 {
                    return true;
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(Solution::max_distance(vec![1, 2, 3, 4, 7], 3), 3);
        assert_eq!(
            Solution::max_distance(vec![5, 4, 3, 2, 1, 1000000000], 2),
            999999999
        );
    }
}
