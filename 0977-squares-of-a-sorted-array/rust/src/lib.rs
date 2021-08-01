struct Solution;

impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        // 设置两个索引，分别指向数组的两端
        // right 指向终止位置
        let mut right = nums.len() - 1;
        // left 指向起始位置
        let mut left = 0;
        // 设置一个新数组 result 用来存放最终的输出结果
        let mut result = vec![0; nums.len()];
        // 让 index 指向 result 数组的终止位置，观察这个位置应该存放什么数字
        let mut index = result.len() - 1;
        // left 向右移动，right 向左移动，当 left 大于 right 时，说明已经观察遍历了 nums 数组中的所有元素，跳出循环
        while left <= right {
            // 说明左边的平方数大于右边
            if nums[left] * nums[left] > nums[right] * nums[right] {
                // result 数组中 index 位置要存放更大的个数，即 nums[left] * nums[left]
                result[index] = nums[left] * nums[left];
                // 由于相对较大的数是在 left 位置，上一行代码已经将它赋值到 index 位置
                // 所以此时 left 位置的数已经失去作用，将它向后移动
                left += 1;
                // 此时，index 位置已经存放好数，将它向前移动，观察下一个位置应该存放哪个数
                index -= 1;
                // 说明右边的平方数大于左边
            } else {
                // result 数组中 index 位置要存放更大的个数，即 nums[right] * nums[right]
                result[index] = nums[right] * nums[right];
                // 由于相对较大的数是在 right 位置，上一行代码已经将它赋值到 index 位置
                // 所以此时 right 位置的数已经失去作用，将它向前移动
                right -= 1;
                // 此时 index 位置已经存放好数，将它向前移动，观察下一个位置应该存放哪个数
                if index < 1 {
                    break;
                }
                index -= 1;
            }
        }
        // 最后返回我们设置的结果数组即可
        result
    }

    pub fn sorted_squares_sort(nums: Vec<i32>) -> Vec<i32> {
        let mut res = nums.iter().map(|x| x * x).collect::<Vec<i32>>();
        res.sort();
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::sorted_squares(vec![-4, -1, 0, 3, 10]), vec![0, 1, 9, 16, 100]);
    }
}
