impl Solution {
    pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
        let nums = merge_sort(nums);
        // 第 k 个最大元素就是第 n -k 个最小元素
        nums[nums.len() - k as usize]
    }
}

pub fn merge_sort(mut nums: Vec<i32>) -> Vec<i32> {
    if nums.is_empty() {
        return nums;
    }

    let n = nums.len() - 1;
    merge_sort_recursion(&mut nums, 0, n);
    nums
}

fn merge_sort_recursion(nums: &mut Vec<i32>, left: usize, right: usize) {
    // 判断是否只剩下最后一个元素
    if left >= right {
        return;
    }

    // 从中间将数组分成两个序列
    let mid = left + (right - left) / 2;

    // 分别以递归方式将左右两个序列排好序
    merge_sort_recursion(nums, left, mid);
    merge_sort_recursion(nums, mid + 1, right);

    // 将已有序的两个序列合并
    merge(nums, left, mid, right);
}

fn merge(nums: &mut Vec<i32>, left: usize, mid: usize, right: usize) {
    // 定义索引 i 表示左序列的起始位置
    let mut i = left;

    // 定义索引 j 表示右序列的起始位置
    let mut j = mid + 1;

    // 定义索引 k 表示开始排序原数组的位置
    let mut k = left;

    // 定义用于排序过程中临时存放元素的数组
    let mut tmp = vec![];

    while k <= right {
        if i > mid {
            // 左序列元素处理完毕，剩下右序列元素，将右序列元素逐个添加
            tmp.push(nums[j]);
            j += 1;
            k += 1;
        } else if j > right {
            // 右序列元素处理完毕，剩下左序列元素，将左序列元素逐个添加
            tmp.push(nums[i]);
            i += 1;
            k += 1;
        } else if nums[i] < nums[j] {
            // 左序列元素小于右序列元素，将左序列元素添加，索引 i 往前移动一位
            tmp.push(nums[i]);
            i += 1;
            k += 1;
        } else {
            // 左序列元素大于右序列元素，将右序列元素添加，索引 j 往前移动一位
            tmp.push(nums[j]);
            j += 1;
            k += 1;
        }
    }

    // 将已排序的元素按对应位置替换原数组元素
    for i in 0..=(right - left) {
        nums[left + i] = tmp[i];
    }
}

struct Solution {}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::find_kth_largest(vec![3, 2, 1, 5, 6, 4], 2), 5);
        assert_eq!(
            Solution::find_kth_largest(vec![3, 2, 3, 1, 2, 4, 5, 5, 6], 4),
            4
        );
    }
}
