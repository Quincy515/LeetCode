impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let (mut left, mut right) = (0i128, x as i128);
        while left <= right {
            let mid = left + (right - left) / 2;
            let mid2 = mid * mid;
            match mid2.cmp(&(x as i128)) {
                std::cmp::Ordering::Equal => return mid as i32,
                std::cmp::Ordering::Less => left = mid + 1,
                std::cmp::Ordering::Greater => right = mid - 1,
            }
        }
        right as i32
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
