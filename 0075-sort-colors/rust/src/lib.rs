struct Solution;

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        if nums.is_empty() {
            return;
        }
        // left 指向数组的开始的位置，它指向的位置左侧都是 0
        let mut left = 0usize;
        // right 指向数组的结束的位置，它指向的位置右侧都是 2
        let mut right = nums.len() - 1;
        // index 指向数组的开始位置
        let mut index = 0;

        // index 向后移动，当它越过 right 时跳出循环，不需要再判断了
        // 因此此时说明 index 右侧的都已经是 2
        while index <= right {
            // 获取当前的元素值
            let cur = nums[index];
            // 如果 index 位置上的元素值为 0
            if cur == 0 {
                // 说明是红色，要放到最前面去
                // 最前面的那个元素被 left 指着，所以让 index 指向的元素和 left 指向位置上的元素进行交换
                nums.swap(left, index);
                // index 可以向后移动
                index += 1;
                // left 可以向右移动，它的左侧区域都是 0
                left += 1;
                // 如果 index 位置上的元素值为 1
            } else if cur == 1 {
                // 说明是白色，就应该放在中间，不用管它，继续移动 index
                index += 1;
                // 如果 index 位置上的元素值为 2
            } else if cur == 2 {
                // 说明是蓝色，要放到最后面
                // 所以让 index 指向的元素和 right 指向位置上的元素进行交换
                nums.swap(index, right);
                // 由于原先 right 指向的元素可能为 0、1、2 这三种的任何一种
                // 交换到了 index 后，还需要继续观察一轮，所以 index 先不移动
                if right < 1 {
                    break;
                }
                right -= 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_75() {
        let mut vec = vec![
            1, 2, 0, 1, 2, 2, 2, 0, 0, 0, 2, 1, 1, 2, 0, 1, 2, 2, 1, 1, 0,
        ];
        Solution::sort_colors(&mut vec);
        assert_eq!(
            vec,
            vec![0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2]
        );

        let mut vec = vec![];
        Solution::sort_colors(&mut vec);
        assert_eq!(vec, vec![]);

        let mut vec = vec![2, 2, 2];
        Solution::sort_colors(&mut vec);
        assert_eq!(vec, vec![2, 2, 2]);
    }
}
