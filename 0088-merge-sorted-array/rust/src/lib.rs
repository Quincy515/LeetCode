struct Solution;

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        // 索引从有序数组 nums1 有效元素的末端开始
        // 数组的下标索引从零开始计数
        // 索引  0  1  2
        // 数组 [1, 2, 3]
        let mut i = m - 1;
        // 索引从有序数组 nums2 的末端开始
        let mut j = n - 1;
        // 从有序数组 nums1 最末端的位置开始保存元素
        let mut cur = m + n - 1;

        // 通过循环把 nums2 的元素都移动到 nums1 中
        while j >= 0 {
            // 比较 nums1 和 nums2 中当前的元素大小
            // 如果 nums1 中的索引位置为 i 的元素大于 nums2 中索引位置为 j 的元素
            // 为了防止越界 i 必须大于等于 0
            if i >= 0 && nums1[i as usize] > nums2[j as usize] {
                // 把 nums1 中的索引位置为 i 的元素复制到索引为 cur 的位置
                // 此时 cur 的元素已经确定下来
                nums1[cur as usize] = nums1[i as usize];
                // 接下来去确定 cur 前面一个元素应该放什么数字
                cur -= 1;
                // 此时，索引 i 需要向前移动
                i -= 1;
                // 否则，如果 nums1 中的索引位置为 i 的元素小于或等于 nums2 中索引位置为 j 的元素
            } else {
                // 把 nums2 中的索引位置为 j 的元素复制到索引为 cur 的位置
                nums1[cur as usize] = nums2[j as usize];
                // 接下来去确定 cur 前面一个元素应该放什么数字
                cur -= 1;
                // 此时，索引 j 需要向前移动
                j -= 1;
            }
        }
    }

    pub fn merge2(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        // 初始化游标i和j分别指向数组最后一个元素
        let mut i = m - 1;
        let mut j =n - 1;
        // 初始化插入游标的位置k
        let mut k = m + n - 1;
        // 当i和j都大于等于0时，不断对比两个数组中的元素
        while i >= 0 && j >= 0 {
            // 如果第二个数组中的元素比较大，则把它插入到位置k
            if nums2[j as usize] > nums1[i as usize] {
                nums1[k as usize] = nums2[j as usize];
                // 然后j和k都向左移动一个位置
                k -= 1;
                j -= 1;
            } else {
                // 否则把第一个数组中的元素插入到位置k，和k都向左移动一个位置
                nums1[k as usize] = nums1[i as usize];
                i -= 1;
                k -= 1;
            }
        }
        // 循环结束后检查第二个数组中是否还有未对比的元素
        while j >= 0 {
            // 如果有则把他们都插入到第一个数组
            nums1[k as usize] = nums2[j as usize];
            k -= 1;
            j -= 1;
        }
        // 至此第一个数组就是合并后的有序数组，Time:O(m+n), Space:O(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_88() {
        let mut vec1 = vec![1, 2, 3, 0, 0, 0];
        let mut vec2 = vec![2, 5, 6];
        Solution::merge(&mut vec1, 3, &mut vec2, 3);
        assert_eq!(vec1, vec![1, 2, 2, 3, 5, 6]);

        let mut vec1 = vec![1, 2, 3];
        let mut vec2 = vec![];
        Solution::merge(&mut vec1, 3, &mut vec2, 0);
        assert_eq!(vec1, vec![1, 2, 3]);

        let mut vec1 = vec![0, 0, 0];
        let mut vec2 = vec![1, 2, 3];
        Solution::merge(&mut vec1, 0, &mut vec2, 3);
        assert_eq!(vec1, vec![1, 2, 3]);
    }
}
