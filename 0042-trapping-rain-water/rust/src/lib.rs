struct Solution;
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let n = height.len();
        let mut result = 0;
        // 遍历每个柱子 h, 查找它左边的最高柱子 lh, 和右边的最高柱子 rh
        // 柱子上能承载的雨水 = min(lh, rh) - h
        for i in 1..n - 1 {
            let mut lh = 0;
            for j in 0..i {
                // 左侧最高 lh
                if height[j] > lh {
                    lh = height[j];
                }
            }
            let mut rh = 0;
            for j in i + 1..n {
                // 右侧最高 rh
                if height[j] > rh {
                    rh = height[j];
                }
            }
            let mut carry = i32::min(lh, rh) - height[i];
            if carry < 0 {
                carry = 0;
            }
            result += carry;
        }
        result
    }

    pub fn trap2(height: Vec<i32>) -> i32 {
        let n = height.len();
        // 前缀 max
        let mut left_max = vec![0; n];
        let mut max = 0;
        for i in 0..n {
            left_max[i] = i32::max(max, height[i]);
            max = left_max[i];
        }
        // 后缀 max
        let mut right_max = vec![0; n];
        max = 0;
        for i in (0..=n - 1).rev() {
            right_max[i] = i32::max(max, height[i]);
            max = right_max[i];
        }
        // 每个柱子之上承载的雨水
        let mut result = 0;
        for i in 1..n - 1 {
            result += i32::min(left_max[i], right_max[i]) - height[i];
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
        assert_eq!(Solution::trap(vec![4, 2, 0, 3, 2, 5]), 9);
    }
}
