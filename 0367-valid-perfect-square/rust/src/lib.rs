impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        // let num = num as i128;
        let (mut left, mut right) = (0_i128, num as i128);
        while left <= right {
            let mid = left + (right - left) / 2;
            let mid2 = mid * mid;
            match mid2.cmp(&(num as i128)) {
                std::cmp::Ordering::Equal => return true,
                std::cmp::Ordering::Greater => right = mid - 1,
                std::cmp::Ordering::Less => left = mid + 1,
            }
        }
        false
    }
}
struct Solution;
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert!(Solution::is_perfect_square(16));
        assert!(!Solution::is_perfect_square(14));
        assert!(!Solution::is_perfect_square(2147483647));
    }
}
