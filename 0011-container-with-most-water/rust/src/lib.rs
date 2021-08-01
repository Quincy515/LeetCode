struct Solution;
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        // 设置两个索引，分别指向容器的两侧
        // 索引 left 指向最左边的柱子
        let mut left = 0;
        // 索引 right 指向最右边的柱子
        let mut right = height.len() - 1;
        // 设置一个变量来保存当下水的最大面积
        let mut res = 0;
        // 移动 left 和 right，直到 left 和 right 相遇为止
        while left < right {
            // 水的宽度 right - left
            let width = (right - left) as i32;
            // 水的高度由两根柱子最短的那根决定
            let h = i32::min(height[left], height[right]);
            // 计算此时水的面积
            let area = width * h;
            // 如果此时水的面积大于了我们之前保存的那个值，我们需要更新一下
            if area >= res {
                // 更新 res 的值为 area，确保 res 一直都是最大的值
                res = area;
            }
            // 接下来去观察需要移动的哪根柱子：必须是最短的那根柱子
            // 如果左边的柱子更短，那么向内移动左边的柱子，因为只有这样，才有可能找到一个更高的水面
            // 在宽度一定变小的情况下，水的面积才有可能增大
            if height[left] < height[right] {
                // 向内移动左边的柱子
                left += 1;
            // 如果有变的柱子更短，那么向内移动右边的柱子，因为只有这样，才有可能找到一个更高的水面
            // 在宽度一定变小的情况下，水的面积才有可能增大
            } else {
                // 向内移动右边的柱子
                right -= 1;
            }
        }
        // 最后返回最大的面积 res 即可
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
        assert_eq!(Solution::max_area(vec![1, 1]), 1);
        assert_eq!(Solution::max_area(vec![4, 3, 2, 1, 4]), 16);
    }
}
